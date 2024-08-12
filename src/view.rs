use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Backend, Terminal},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use crate::constants::Screen;
use crate::state::State;

#[derive(Debug, Default)]
pub struct View {}

impl View {
    pub fn draw<B: Backend>(
        terminal: &mut Terminal<B>,
        state: &State,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|frame| {
            let area = frame.size();

            match state.screen {
                Screen::Main => View::draw_main_scene(frame, area, state),
                Screen::Add => View::draw_add_task_scene(frame, area, state),
                Screen::Greetings => View::draw_greetings_scene(frame, area, state),
            }
        })?;
        Ok(())
    }

    fn get_chunks(area: Rect) -> (Rc<[Rect]>, Rc<[Rect]>) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer_layout[1]);

        return (outer_layout, inner_layout);
    }

    fn draw_greetings_scene(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let message = if state.get_is_first_time() {
            format!(
            "Hello there 👋, let's set a master key with numbers \nthat is 10 char max and promise you will never forget!\n{}
            ",
            String::from("*").repeat(state.master_key.len())
            )
        } else {
            format!(
                "Hello there 👋, enter your master key please\n{}",
                String::from("*").repeat(state.master_key.len())
            )
        };
        let widget = Paragraph::new(message)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL).title("tudu"));

        frame.render_widget(widget, outer_layout[0]);
        View::draw_legend(frame, "esc: Cancel, enter: Enter", inner_layout[0]);
        View::draw_error(frame, &state, inner_layout[1]);
    }

    fn draw_main_scene(frame: &mut Frame, area: Rect, state: &State) {
        let selected_line = state.line;

        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let items: Vec<Span> = state
            .task_list
            .iter()
            .enumerate()
            .map(|e| {
                let checkbox = if e.1.status == "completed" {
                    '✓'
                } else {
                    ' '
                };
                let content = format!(" [{}] {} :: {}", checkbox, e.1.title, e.1.status);
                if e.0 as i32 == selected_line {
                    Span::styled(content, Style::default().bg(Color::LightYellow))
                } else {
                    Span::raw(content)
                }
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Task List"))
            .style(Style::default().fg(Color::White));
        frame.render_widget(list, outer_layout[0]);

        View::draw_legend(
            frame,
            "esc: Exit, a: Add, x: Remove, enter: Check/Uncheck, ↑: Up, ↓: Down",
            inner_layout[0],
        );
        View::draw_error(frame, &state, inner_layout[1]);
    }

    fn draw_add_task_scene(frame: &mut Frame, area: Rect, state: &State) {
        let content = &state.input;

        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let input_field = Paragraph::new(String::from(content))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Write the task, max 40 characters"),
            );

        frame.render_widget(input_field, outer_layout[0]);

        View::draw_legend(frame, "esc: Cancel, enter: Save", inner_layout[0]);
        View::draw_error(frame, &state, inner_layout[1]);
    }

    fn draw_legend(frame: &mut Frame, text: &str, area: Rect) {
        let widget = Paragraph::new(text)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(widget, area);
    }

    fn draw_error(frame: &mut Frame, state: &State, area: Rect) {
        let widget = Paragraph::new(state.get_error().as_str())
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(widget, area);
    }
}
