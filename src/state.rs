use crate::constants::Screen;
use crate::task::Task;

#[derive(Debug, Default)]
pub struct State {
    pub task_list: Vec<Task>,
    pub running: bool,
    pub screen: Screen,
    pub line: i32,
}
