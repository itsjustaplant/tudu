pub const APP_PATH: &str = "tudu";
pub const DB_NAME: &str = "tudu.db";

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Add,
}

pub const MAX_TASK_TITLE_LENGTH: i32 = 40;
