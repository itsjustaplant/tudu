pub const APP_PATH: &str = "tudu";
pub const DB_NAME: &str = "tudu.db";

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Add,
    Greetings,
}

pub const MAX_TASK_TITLE_LENGTH: i32 = 40;
pub const MAX_MASTER_KEY_LENGTH: i32 = 10;
