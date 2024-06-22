use crate::constants::Screen;
use crate::task::Task;

#[derive(Debug, Default)]
pub struct State {
    pub task_list: Vec<Task>,
    pub running: bool,
    pub screen: Screen,
    pub line: i32,
    pub input: String,
    pub error: String,
}

impl State {
    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    pub fn get_input(&self) -> &String {
        &self.input
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
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

    pub fn get_running(&self) -> bool {
        self.running
    }

    pub fn set_running(&mut self, new_state: bool) {
        self.running = new_state;
    }

    pub fn get_task_list(&self) -> &Vec<Task> {
        &self.task_list
    }

    pub fn set_task_list(&mut self, task_list: Vec<Task>) {
        self.task_list = task_list;
    }
}
