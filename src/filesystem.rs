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

pub fn create_config_folder() -> Result<(), Error> {
    let app_config_path = get_app_config_path()?;

    match fs::create_dir_all(&app_config_path) {
        Ok(()) => Ok(()),
        Err(_) => Err(Error::new(
            ErrorKind::Other,
            "Could not create config folder",
        )),
    }
}

pub fn db_exists() -> bool {
    let app_config_path = get_app_config_path();
    match app_config_path {
        Ok(mut acp) => {
            acp.push(constants::DB_NAME);
            fs::metadata(&acp).is_ok()
        }
        Err(_) => {
            println!("nope");
            false
        }
    }
}
