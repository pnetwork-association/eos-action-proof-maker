use crate::{constants::LOG_FILE_PATH, error::AppError, state::State, types::Result};
use chrono::Utc;
use log::LevelFilter;
use simplelog::*;
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

fn get_log_file_path() -> String {
    format!("{}{}.log", LOG_FILE_PATH, Utc::now())
}

pub fn initialize_logger(state: State) -> Result<State> {
    let log_path = get_log_file_path();
    if !Path::new(&LOG_FILE_PATH).exists() {
        info!("✔ No log dir found, creating...");
        create_dir_all(LOG_FILE_PATH)?;
    };
    match WriteLogger::init(
        LevelFilter::Trace,
        Config::default(),
        File::create(log_path.clone())?,
    ) {
        Ok(_) => {
            info!("✔ Logger initialized successfully");
            info!("✔ Log writing to: {}", log_path);
            Ok(state)
        }
        Err(e) => Err(AppError::Custom(e.to_string())),
    }
}
