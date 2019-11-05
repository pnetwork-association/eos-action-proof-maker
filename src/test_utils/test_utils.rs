use std::{
    path::Path,
    fs::read_to_string,
};
use crate::{
    error::AppError,
    types::{
        Result,
        EosInputJson,
    },
};

pub const SAMPLE_SUBMISSION_JSON_PATH: &str =
"src/test_utils/sample-submission-json";

pub fn get_sample_submission_string() -> Result<String> {
    match Path::new(&SAMPLE_SUBMISSION_JSON_PATH).exists() {
        true => Ok(read_to_string(SAMPLE_SUBMISSION_JSON_PATH)?),
        false => Err(AppError::Custom(
            format!("✘ Cannot find sample-submission-json file!")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_sample_submission_json_string() {
        let result = get_sample_submission_string()
            .unwrap();
        println!("{}",result);
    }
}
