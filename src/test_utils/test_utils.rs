use std::{
    path::Path,
    fs::read_to_string,
};
use crate::{
    error::AppError,
    parse_input_json::parse_eos_input_json_string,
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

pub fn get_sample_submission_json() -> Result<EosInputJson> {
     parse_eos_input_json_string(
         &get_sample_submission_string()?
     )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_sample_submission_json_string() {
        if let Err(e) = get_sample_submission_string() {
            panic!("Error getting sample submission string: {}", e)
        }
    }

    #[test]
    fn should_get_sample_eos_json() {
        if let Err(e) = get_sample_submission_json() {
            panic!("Error getting sample submission json: {}", e)
        }
    }
}
