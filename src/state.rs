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
