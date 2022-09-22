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

pub fn initialize_logger(state: State) -> Result<State> {
    let log_path = get_log_file_path();
    if !Path::new(&LOG_FILE_PATH).exists() {
        info!("✔ No log dir found, creating...");
        create_dir_all(LOG_FILE_PATH)?;
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
