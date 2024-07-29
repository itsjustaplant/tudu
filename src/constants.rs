pub const APP_PATH: &str = "tudu";
pub const DB_NAME: &str = "tudu.db";

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Add,
    Greetings,
}

pub enum Action {
    Empty,
    Exit,
    GetTasks,
    OpenMainScreen,
    OpenAddScreen,
    OpenGreetingsScreen,
    AddTask,
    CancelAddTask,
    RemoveTask,
    InputChar(char),
    InputMaskedChar(char),
    RemoveChar,
    RemoveMaskedChar,
    MenuUp,
    MenuDown,
    ToggleTaskStatus,
    ResetError,
    AddSecret,
    CheckSecret,
}

pub const MAX_TASK_TITLE_LENGTH: i32 = 40;
pub const MAX_MASTER_KEY_LENGTH: i32 = 10;
pub const VERY_SECRET_TEXT: &str = "THIS_IS_NOT_GOOD_PRACTICE_I_NEED_TIME_FOR_THIS";
