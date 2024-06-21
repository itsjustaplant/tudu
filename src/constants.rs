pub const APP_PATH: &str = "todocli";
pub const DB_NAME: &str = "todoclidb.db";

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Add,
}
