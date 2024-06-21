use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Backend, Terminal},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph},
};

use crate::constants::Screen;
use crate::state::State;

#[derive(Debug, Default)]
pub struct View {}

impl View {
    pub fn temp(self) {
        println!("he");
    }

    pub fn draw<B: Backend>(
        terminal: &mut Terminal<B>,
        state: &State,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|frame| {
            let area = frame.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(area);

            match state.screen {
                Screen::Main => {
                    let items: Vec<String> = state
                        .task_list
                        .iter()
                        .map(|task| format!(" [ ] {} :: {}", task.title, task.status))
                        .collect();
                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Task List"))
                        .style(Style::default().fg(Color::White));
                    frame.render_widget(list, chunks[0]);

                    let bottom_text =
                        Paragraph::new("q: Exit, a: Add, x: Remove, space: Check/Uncheck")
                            .alignment(Alignment::Left)
                            .block(Block::default().borders(Borders::NONE));

                    frame.render_widget(bottom_text, chunks[1]);
                }
                _ => print!("yay"),
            }
        })?;
        Ok(())
    }
}
