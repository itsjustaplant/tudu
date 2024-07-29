use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};

use crate::client::Client;
use crate::constants::{self, Action, Screen, MAX_TASK_TITLE_LENGTH, VERY_SECRET_TEXT};
use crate::encdec::{decrypt, encrypt};
use crate::filesystem;
use crate::state::State;
use crate::view::View;

pub struct Controller {
    pub state: State,
    client: Client,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            client: Client::default(),
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Exit => self.state.set_is_running(false),
            Action::GetTasks => {
                match self.client.get_tasks() {
                    Ok(mut task_list) => {
                        for task in task_list.iter_mut() {
                            task.title =
                                decrypt(task.title.as_str(), &self.state.get_master_key()).unwrap();
                        }
                        self.state.set_task_list(task_list);
                        self.handle_action(Action::ResetError);
                    }
                    Err(e) => {
                        self.state.set_error(format!("{}", e));
                    }
                };
            }
            Action::MenuDown => {
                let current_line = self.state.get_line();
                if current_line < self.state.get_task_list_length() - 1 {
                    self.state.set_line(current_line + 1);
                }
            }
            Action::MenuUp => {
                let current_line = self.state.get_line();
                if current_line > 0 {
                    self.state.set_line(current_line - 1);
                }
            }
            Action::OpenMainScreen => {
                self.state.set_screen(Screen::Main);
                self.handle_action(Action::CheckSecret);
            }
            Action::OpenAddScreen => {
                self.state.set_screen(Screen::Add);
                self.handle_action(Action::ResetError);
            }
            Action::OpenGreetingsScreen => {
                self.state.set_screen(Screen::Greetings);
            }
            Action::CancelAddTask => {
                self.state.set_screen(Screen::Main);
                self.handle_action(Action::ResetError);
            }
            Action::InputChar(ch) => {
                let len = self.state.input.len();
                self.state.input.insert(len, ch);
            }
            Action::InputMaskedChar(ch) => {
                let len = self.state.master_key.len();

                if len as i32 <= constants::MAX_MASTER_KEY_LENGTH {
                    self.state.master_key.insert(len, ch);
                }
            }
            Action::RemoveChar => {
                let len = self.state.input.len();
                if len > 0 {
                    self.state.input.drain(len - 1..len);
                }
            }
            Action::RemoveMaskedChar => {
                let len = self.state.master_key.len();
                if len > 0 {
                    self.state.master_key.drain(len - 1..len);
                }
            }
            Action::AddTask => match self.state.input.len() {
                0 => self
                    .state
                    .set_error(String::from("Please enter task title")),
                len if len as i32 > MAX_TASK_TITLE_LENGTH => self.state.set_error(String::from(
                    format!("Task title cannot be longer than {}", MAX_TASK_TITLE_LENGTH),
                )),
                _ => {
                    let data = encrypt(&self.state.input, &self.state.get_master_key());
                    match self.client.create_task(data) {
                        Ok(_) => {
                            self.state.set_input("");
                            self.handle_action(Action::OpenMainScreen);
                        }
                        Err(e) => self.state.set_error(format!("{}", e)),
                    }
                }
            },
            Action::AddSecret => {
                let data = encrypt(VERY_SECRET_TEXT, &self.state.get_master_key());
                match self.client.create_user(data) {
                    Ok(_) => self.handle_action(Action::OpenMainScreen),
                    Err(e) => self.state.set_error(format!("{}", e)),
                }
            }
            Action::CheckSecret => {
                let user_data = self.client.get_user();

                match user_data {
                    Ok(user_vec) => {
                        let user_result = user_vec.get(0);
                        match user_result {
                            Some(user) => {
                                let decrypted_text =
                                    decrypt(user.secret.as_str(), &self.state.get_master_key());
                                match decrypted_text {
                                    Ok(_) => {
                                        self.handle_action(Action::ResetError);
                                        self.handle_action(Action::GetTasks);
                                    }
                                    Err(_) => {
                                        self.state.set_error(String::from("Password is wrong"));
                                        self.handle_action(Action::OpenGreetingsScreen);
                                    }
                                }
                            }
                            None => self.state.set_error(String::from("Could not get user")),
                        }
                    }
                    Err(_) => self.state.set_error(String::from("Could not get user")),
                }
            }
            Action::RemoveTask => {
                let index = self.state.get_line();
                self.state.get_task_list().get(index as usize).map(|task| {
                    // TODO: propagate this error
                    let _ = self.client.remove_task(task.id);
                });
                if index == self.state.get_task_list_length() - 1 {
                    self.handle_action(Action::MenuUp)
                }
                self.handle_action(Action::GetTasks);
            }
            Action::ToggleTaskStatus => {
                let index = self.state.get_line();
                self.state.get_task_list().get(index as usize).map(|task| {
                    // TODO: propagate this error
                    let _ = self.client.update_task(task.id, &task.status);
                });
                self.handle_action(Action::GetTasks);
            }
            Action::ResetError => {
                self.state.set_error(String::from(""));
            }
            Action::Empty => {}
        }
    }

    pub fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = match self.state.get_screen() {
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
                        Screen::Greetings => match key.code {
                            KeyCode::Esc => Action::Exit,
                            KeyCode::Char(to_insert) => Action::InputMaskedChar(to_insert),
                            KeyCode::Backspace => Action::RemoveMaskedChar,
                            KeyCode::Enter => {
                                if self.state.get_is_first_time() {
                                    Action::AddSecret
                                } else {
                                    Action::OpenMainScreen
                                }
                            }
                            _ => Action::Empty,
                        },
                    };
                    self.handle_action(action);
                }
            }
        }
        Ok(())
    }

    pub fn init_controller(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let is_first_time = !filesystem::db_exists();

        self.state.set_is_first_time(is_first_time);

        filesystem::create_config_folder()?;
        self.handle_action(Action::OpenGreetingsScreen);
        self.client.open_connection()?;
        self.client.create_user_table()?;
        self.client.crete_todos_table()?;
        self.state.set_is_running(true);
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

        while self.state.get_is_running() {
            self.handle_events()?;
            View::draw(terminal, &self.state)?;
        }
        Ok(())
    }
}
