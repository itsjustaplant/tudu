use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};

use crate::client::Client;
use crate::constants::Screen;
use crate::filesystem;
use crate::state::State;
use crate::view::View;

pub enum Action {
    Empty,
    Exit,
    GetTasks,
    OpenMainScreen,
    OpenAddScreen,
    AddTask,
    CancelAddTask,
    RemoveTask,
    InputChar(char),
    RemoveChar,
    MenuUp,
    MenuDown,
    ToggleTaskStatus,
}

pub struct Controller {
    pub state: State,
    client: Client,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            client: Client::default(),
        }
    }

    pub fn handle_action(&mut self, action: Action) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            Action::Exit => self.state.running = false,
            Action::GetTasks => {
                self.state.task_list = self.client.get_tasks()?;
            }
            Action::MenuDown => {
                if self.state.line < self.state.task_list.len() as i32 - 1 {
                    self.state.line += 1;
                }
            }
            Action::MenuUp => {
                if self.state.line > 0 {
                    self.state.line -= 1;
                }
            }
            Action::OpenMainScreen => {
                self.state.screen = Screen::Main;
                self.handle_action(Action::GetTasks)?;
            }
            Action::OpenAddScreen => self.state.screen = Screen::Add,
            Action::CancelAddTask => {
                self.state.input = String::from("");
                self.state.screen = Screen::Main;
            }
            Action::InputChar(ch) => {
                let len = self.state.input.len();
                if len < 80 {
                    self.state.input.insert(len, ch);
                }
            }
            Action::RemoveChar => {
                let len = self.state.input.len();
                self.state.input.drain(len - 1..len);
            }
            Action::AddTask => {
                self.client.create_task(&self.state.input)?;
                self.state.input = String::from("");
                self.handle_action(Action::OpenMainScreen)?;
            }
            Action::RemoveTask => {
                let index = self.state.line;
                self.state.task_list.get(index as usize).map(|task| {
                    // TODO: propagate this error
                    let _ = self.client.remove_task(task.id);
                });
                self.handle_action(Action::GetTasks)?;
            }
            Action::ToggleTaskStatus => {
                let index = self.state.line;
                self.state.task_list.get(index as usize).map(|task| {
                    // TODO: propagate this error
                    let _ = self.client.update_task(task.id, &task.status);
                });
                self.handle_action(Action::GetTasks)?;
            }
            Action::Empty => {}
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = match self.state.screen {
                        Screen::Main => match key.code {
                            KeyCode::Char('a') => Action::OpenAddScreen,
                            KeyCode::Char('x') => Action::RemoveTask,
                            KeyCode::Up => Action::MenuUp,
                            KeyCode::Down => Action::MenuDown,
                            KeyCode::Esc => Action::Exit,
                            KeyCode::Enter => Action::ToggleTaskStatus,
                            _ => Action::Empty,
                        },
                        Screen::Add => match key.code {
                            KeyCode::Esc => Action::CancelAddTask,
                            KeyCode::Enter => Action::AddTask,
                            KeyCode::Char(to_insert) => Action::InputChar(to_insert),
                            KeyCode::Backspace => Action::RemoveChar,
                            _ => Action::Empty,
                        },
                    };
                    self.handle_action(action)?;
                }
            }
        }
        Ok(())
    }

    pub fn init_controller(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        filesystem::create_config_folder()?;
        self.client.open_connection()?;
        self.client.crete_todos_table()?;
        self.state.running = true;
        self.handle_action(Action::OpenMainScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.close_connection()?;
        Ok(())
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.init_controller()?;

        while self.state.running {
            self.handle_events()?;
            View::draw(terminal, &self.state)?;
        }
        Ok(())
    }
}
