use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};

use crate::client::Client;
use crate::constants::{self, Action, Screen, MAX_TASK_TITLE_LENGTH, VERY_SECRET_TEXT};
use crate::encdec::{decrypt, encrypt};
use crate::filesystem::{self, get_app_config_path};
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
            Action::Init => self.state.set_is_running(true),
            Action::Exit => {
                self.state.set_is_running(false);
                self.exit().expect("Could not exit");
            }
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

    pub fn handle_key_stroke(&self, key_code: KeyCode) -> Action {
        return match self.state.get_screen() {
            Screen::Main => match key_code {
                KeyCode::Char('a') => Action::OpenAddScreen,
                KeyCode::Char('x') => Action::RemoveTask,
                KeyCode::Up => Action::MenuUp,
                KeyCode::Down => Action::MenuDown,
                KeyCode::Esc => Action::Exit,
                KeyCode::Enter => Action::ToggleTaskStatus,
                _ => Action::Empty,
            },
            Screen::Add => match key_code {
                KeyCode::Esc => Action::CancelAddTask,
                KeyCode::Enter => Action::AddTask,
                KeyCode::Char(to_insert) => Action::InputChar(to_insert),
                KeyCode::Backspace => Action::RemoveChar,
                _ => Action::Empty,
            },
            Screen::Greetings => match key_code {
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
    }

    pub fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = self.handle_key_stroke(key.code);
                    self.handle_action(action);
                }
            }
        }
        Ok(())
    }

    pub fn init_controller(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let app_config_path = get_app_config_path()?;
        let is_first_time = !filesystem::db_file_exists(&app_config_path, constants::DB_NAME);

        self.state.set_is_first_time(is_first_time);

        filesystem::create_config_folder(&app_config_path)?;
        self.handle_action(Action::OpenGreetingsScreen);
        self.client.open_connection(app_config_path, constants::DB_NAME)?;
        self.client.create_user_table()?;
        self.client.crete_todos_table()?;
        self.handle_action(Action::Init);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_action_handler() {
        let mut path = PathBuf::new();
        path.push("./test/controller/");

        let mut controller = Controller::new();
        assert_eq!(controller.state.get_is_running(), false);

        controller
            .client
            .open_connection(path, constants::DB_NAME)
            .expect("Could not open connection");

        // Greetings
        controller.handle_action(Action::OpenGreetingsScreen);
        assert_eq!(controller.state.get_screen(), &Screen::Greetings);

        controller.handle_action(Action::InputMaskedChar('S'));
        controller.handle_action(Action::InputMaskedChar('E'));
        controller.handle_action(Action::InputMaskedChar('C'));
        controller.handle_action(Action::InputMaskedChar('R'));
        controller.handle_action(Action::InputMaskedChar('E'));
        controller.handle_action(Action::InputMaskedChar('T'));
        controller.handle_action(Action::InputMaskedChar('T'));
        assert_eq!(controller.state.get_master_key(), &String::from("SECRETT"));

        controller.handle_action(Action::RemoveMaskedChar);
        assert_eq!(controller.state.get_master_key(), &String::from("SECRET"));

        // Add secret
        controller.state.set_master_key(String::from("SECRET"));
        controller.handle_action(Action::OpenMainScreen);
        controller.handle_action(Action::Init);
        assert_eq!(controller.state.get_is_running(), true);

        // No item
        controller.handle_action(Action::MenuDown);
        assert_eq!(controller.state.get_line(), 0);

        controller.handle_action(Action::GetTasks);
        let tasks = controller.state.get_task_list();
        assert_eq!(tasks.len(), 0);

        controller.handle_action(Action::AddTask);
        assert_eq!(controller.state.get_error(), "Please enter task title");

        // 2 item
        controller.handle_action(Action::InputChar('c'));
        controller.handle_action(Action::AddTask);
        controller.handle_action(Action::InputChar('c'));
        controller.handle_action(Action::AddTask);
        controller.handle_action(Action::GetTasks);
        controller.handle_action(Action::MenuDown);
        assert_eq!(controller.state.get_line(), 1);
        controller.handle_action(Action::MenuUp);
        assert_eq!(controller.state.get_line(), 0);
        controller.handle_action(Action::ToggleTaskStatus);
        assert_eq!(
            controller
                .state
                .get_task_list()
                .get(0)
                .expect("nope")
                .status,
            "completed"
        );

        // Check remove char
        controller.handle_action(Action::InputChar('c'));
        controller.handle_action(Action::RemoveChar);
        assert_eq!(controller.state.get_input(), &String::from(""));

        // Check task title length
        for _ in 0..41 {
            controller.handle_action(Action::InputChar('c'));
        }
        controller.handle_action(Action::AddTask);
        assert_eq!(
            controller.state.get_error(),
            &String::from("Task title cannot be longer than 40")
        );

        // Remote all items
        controller.handle_action(Action::RemoveTask);
        controller.handle_action(Action::RemoveTask);
        assert_eq!(controller.state.get_line(), 0);

        // Add task
        controller.state.set_error(String::from("ERROR"));
        controller.handle_action(Action::OpenAddScreen);
        assert_eq!(controller.state.get_screen(), &Screen::Add);
        assert_eq!(controller.state.get_error(), &String::from(""));

        // Cancel add task
        controller.handle_action(Action::CancelAddTask);
        assert_eq!(controller.state.get_screen(), &Screen::Main);

        controller.state.set_error(String::from(""));
        controller.state.set_master_key(String::from("MASTER_KEY"));
        controller.handle_action(Action::AddSecret);
        assert_eq!(controller.state.get_screen(), &Screen::Main);
        assert_eq!(controller.state.get_error(), &String::from(""));

        controller.state.set_master_key(String::from(""));
        controller.handle_action(Action::AddSecret);
        assert_eq!(
            controller.state.get_error(),
            &String::from("Password is wrong")
        );

        controller
            .client
            .remove_user()
            .expect("Could not remove user");
        controller.handle_action(Action::Empty);
        controller.handle_action(Action::Exit);
    }

    #[test]
    fn test_key_stroke_handler() {
        let mut controller = Controller::new();

        // Main screen
        let mut action = controller.handle_key_stroke(KeyCode::Char('a'));
        assert_eq!(action, Action::OpenAddScreen);
        action = controller.handle_key_stroke(KeyCode::Char('x'));
        assert_eq!(action, Action::RemoveTask);
        action = controller.handle_key_stroke(KeyCode::Up);
        assert_eq!(action, Action::MenuUp);
        action = controller.handle_key_stroke(KeyCode::Down);
        assert_eq!(action, Action::MenuDown);
        action = controller.handle_key_stroke(KeyCode::Esc);
        assert_eq!(action, Action::Exit);
        action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::ToggleTaskStatus);
        action = controller.handle_key_stroke(KeyCode::Char('z'));
        assert_eq!(action, Action::Empty);

        // Add screen
        controller.handle_action(Action::OpenAddScreen);
        action = controller.handle_key_stroke(KeyCode::Esc);
        assert_eq!(action, Action::CancelAddTask);
        action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::AddTask);
        action = controller.handle_key_stroke(KeyCode::Char('s'));
        assert_eq!(action, Action::InputChar('s'));
        action = controller.handle_key_stroke(KeyCode::Backspace);
        assert_eq!(action, Action::RemoveChar);
        action = controller.handle_key_stroke(KeyCode::Home);
        assert_eq!(action, Action::Empty);

        // Greetings screen
        controller.handle_action(Action::OpenGreetingsScreen);
        action = controller.handle_key_stroke(KeyCode::Esc);
        assert_eq!(action, Action::Exit);
        action = controller.handle_key_stroke(KeyCode::Char('s'));
        assert_eq!(action, Action::InputMaskedChar('s'));
        action = controller.handle_key_stroke(KeyCode::Backspace);
        assert_eq!(action, Action::RemoveMaskedChar);
        action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::OpenMainScreen);
        action = controller.handle_key_stroke(KeyCode::Home);
        assert_eq!(action, Action::Empty);

        controller.state.set_is_first_time(true);
        action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::AddSecret);
    }

    #[test]
    fn test_common_error_tests() {
        let mut controller = Controller::new();

        controller.handle_action(Action::GetTasks);
        assert_ne!(controller.state.get_error(), "");

        controller.state.set_input("TEST TASK TITLE");
        controller.handle_action(Action::AddTask);
        assert_ne!(controller.state.get_error(), "");

        controller.handle_action(Action::CheckSecret);
        assert_eq!(controller.state.get_error(), "Could not get user");
    }

    // #[test]
    // fn test_handle_key_events() {
    //     let mut controller = Controller::new();

    //     let result = controller.handle_events();
    //     assert!(result.is_ok());
    // }
}
