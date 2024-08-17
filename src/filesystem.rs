use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use dirs;

use crate::constants;

pub fn get_app_config_path() -> Result<PathBuf, Error> {
    dirs::config_dir()
        .map(|config_directory| {
            let mut path = PathBuf::from(config_directory);
            path.push(constants::APP_PATH);
            path
        })
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not get config directory path"))
}

pub fn create_config_folder(app_config_path: &PathBuf) -> Result<(), Error> {
    match fs::create_dir_all(&app_config_path) {
        Ok(()) => Ok(()),
        Err(_) => Err(Error::new(
            ErrorKind::Other,
            "Could not create config folder",
        )),
    }
}

pub fn db_file_exists(app_config_path: &PathBuf, db_name: &str) -> bool {
    let absolute_path = app_config_path.join(db_name);
    fs::metadata(&absolute_path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_file_exists() {
        let mut path = PathBuf::new();
        path.push("./test/filesystem/");

        let correct_result = db_file_exists(&path, "tudu.db");
        let wrong_result = db_file_exists(&path, "tudu1.db");

        assert_eq!(correct_result, true);
        assert_eq!(wrong_result, false);
    }

    #[test]
    fn test_config_path() {
        let app_config_path = get_app_config_path();
        assert!(app_config_path.is_ok());
    }

    #[test]
    fn test_return_ok_if_folder_exist() {
        let mut path = PathBuf::new();
        path.push("./test/filesystem/");

        let result = create_config_folder(&path);
        assert!(result.is_ok());
    }
}
