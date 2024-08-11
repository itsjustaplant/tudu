use crate::constants::Screen;
use crate::task::Task;

#[derive(Debug, Default)]
pub struct State {
    pub task_list: Vec<Task>,
    pub is_running: bool,
    pub screen: Screen,
    pub line: i32,
    pub input: String,
    pub error: String,
    pub is_first_time: bool,
    pub master_key: String,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn get_error(&self) -> &String {
        &self.error
    }

    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    pub fn get_input(&self) -> &String {
        &self.input
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = String::from(input);
    }

    pub fn get_line(&self) -> i32 {
        self.line
    }

    pub fn set_line(&mut self, line: i32) {
        self.line = line;
    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.screen = screen;
    }

    pub fn get_is_running(&self) -> bool {
        self.is_running
    }

    pub fn set_is_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }

    pub fn get_task_list(&self) -> &Vec<Task> {
        &self.task_list
    }

    pub fn set_task_list(&mut self, task_list: Vec<Task>) {
        self.task_list = task_list;
    }

    pub fn get_task_list_length(&self) -> i32 {
        self.get_task_list().len() as i32
    }

    pub fn get_is_first_time(&self) -> bool {
        self.is_first_time
    }

    pub fn set_is_first_time(&mut self, is_first_time: bool) {
        self.is_first_time = is_first_time;
    }

    pub fn get_master_key(&self) -> &String {
        &self.master_key
    }

    pub fn set_master_key(&mut self, master_key: String) {
        self.master_key = master_key
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn setter_getter_test() {
    let mut state = State::new();
    let error = String::from("error");
    let input = "input";
    let line = 0;
    let screen = Screen::Greetings;
    let is_running = true;
    let task = Task {
      id: 0,
      title: String::from("title"),
      status: String::from("in-progress")
    };
    let task_list = vec![task];
    let is_first_time = true;
    let master_key = String::from("master_key");


    // setters
    state.set_error(error.clone());
    state.set_input(input);
    state.set_line(line);
    state.set_screen(Screen::Greetings);
    state.set_is_running(is_running);
    state.set_task_list(task_list.clone());
    state.set_is_first_time(is_first_time);
    state.set_master_key(master_key.clone());

    //getters
    assert_eq!(state.get_error(), &error);
    assert_eq!(state.get_input(), input);
    assert_eq!(state.get_line(), line);
    assert_eq!(state.get_screen(), &screen);
    assert_eq!(state.get_is_running(), is_running);
    assert_eq!(state.get_task_list(), &task_list);
    assert_eq!(state.get_task_list_length(), task_list.len() as i32);
    assert_eq!(state.get_is_first_time(), is_first_time);
    assert_eq!(state.get_master_key(), &master_key);
  }
}
