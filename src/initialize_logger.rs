use chrono::Utc;
use simplelog::*;
use log::LevelFilter;
use std::{
    path::Path,
    fs::{
        File,
        create_dir_all,
    },
};
use crate::{
    state::State,
    types::Result,
    error::AppError,
    constants::LOG_FILE_PATH,
};

fn get_log_file_path() -> String {
    format!("{}{}.log", LOG_FILE_PATH, Utc::now())
}

fn create_log_file(path: &String) -> Result<File> {
    Ok(File::create(path)?)
}

pub fn initialize_logger(state: State) -> Result<State> {
    let log_path = get_log_file_path();
    if !Path::new(&LOG_FILE_PATH).exists() {
        info!("✔ No log dir found, creating...");
        create_dir_all(&LOG_FILE_PATH)?;
    };
    match WriteLogger::init(
        LevelFilter::Trace,
        Config::default(),
        File::create(log_path.clone())?
    ) {
        Ok(_) => {
            info!("✔ Logger initialized successfully");
            info!("✔ Log writing to: {}", log_path);
            Ok(state)
        },
        Err(e) => Err(AppError::Custom(e.to_string()))
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn should_get_log_path() {
        let result = get_log_file_path();
        assert!(result.contains("logs/"));
    }

    #[test]
    fn should_create_log_file() {
        let path = get_log_file_path();
        assert!(!Path::new(&path).exists());
        create_log_file(&path)
            .unwrap();
        assert!(Path::new(&path).exists());
        remove_file(path.clone())
            .unwrap();
        assert!(!Path::new(&path).exists());
    }
}
*/
