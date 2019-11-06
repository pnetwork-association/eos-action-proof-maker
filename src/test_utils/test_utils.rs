use std::{
    path::Path,
    fs::read_to_string,
};
use crate::{
    error::AppError,
    parse_eos_block::parse_eos_block_json,
    parse_input_json::parse_eos_input_json_string,
    parse_eos_action_receipts::parse_action_receipt_jsons,
    types::{
        Result,
        EosBlock,
        EosInputJson,
        EosActionReceipts,
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

pub fn get_sample_eos_block() -> Result<EosBlock> {
    get_sample_submission_json()
        .and_then(|json| parse_eos_block_json(&json.block))
}

pub fn get_sample_action_receipts() -> Result<EosActionReceipts> {
    get_sample_submission_json()
        .and_then(|json| parse_action_receipt_jsons(&json.action_receipts))
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

    #[test]
    fn should_get_sample_action_receipts() {
        if let Err(e) = get_sample_action_receipts() {
            panic!("Error getting sample action receipts: {}", e)
        }
    }
}
