use eos_primitives::Action as EosAction;
use std::{
    path::Path,
    fs::read_to_string,
};
use crate::{
    error::AppError,
    parse_eos_block::parse_eos_block_json,
    parse_eos_action::parse_eos_action_json,
    parse_input_json::parse_eos_input_json_string,
    parse_eos_action_receipts::parse_action_receipt_jsons,
    generate_proof::generate_merkle_proof_from_action_receipts,
    parse_eos_action_receipts::sort_action_receipts_by_global_sequence,
    types::{
        Result,
        EosBlock,
        MerkleProof,
        EosInputJson,
        EosActionReceipts,
    },
};

pub const NUM_SAMPLES: usize = 1;
pub const MERKLE_PROOF_INDEX: u32 = 3;

pub fn get_sample_submission_string_n(n: usize) -> Result<String> {
    let path = format!("src/test_utils/sample-block-{}.json", n);
    match Path::new(&path).exists() {
        true => Ok(read_to_string(path)?),
        false => Err(AppError::Custom(
            format!("✘ Cannot find sample-submission-json file!")
        ))
    }
}

pub fn get_sample_submission_json_n(n: usize) -> Result<EosInputJson> {
    parse_eos_input_json_string(&get_sample_submission_string_n(n)?)
}

pub fn get_sample_eos_block_n(n: usize) -> Result<EosBlock> {
    get_sample_submission_json_n(n)
        .and_then(|json| parse_eos_block_json(&json.block))
}

pub fn get_sample_action_receipts_n(n: usize) -> Result<EosActionReceipts> {
    get_sample_submission_json_n(n)
        .and_then(|json| parse_action_receipt_jsons(&json.action_receipts))
        .map(|receipts| sort_action_receipts_by_global_sequence(&receipts))
}

pub fn get_sample_action_n(n: usize) -> Result<EosAction> {
    get_sample_submission_json_n(n)
        .and_then(|json| parse_eos_action_json(&json.action))
}

pub fn get_sample_merkle_proof_n(n: usize) -> Result<MerkleProof> {
    get_sample_action_receipts_n(n)
        .and_then(|receipts|
            generate_merkle_proof_from_action_receipts(
                MERKLE_PROOF_INDEX,
                &receipts,
            )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_sample_submission_json_string() {
        if let Err(e) = get_sample_submission_string_n(1) {
            panic!("Error getting sample submission string: {}", e)
        }
    }

    #[test]
    fn should_get_sample_eos_json() {
        if let Err(e) = get_sample_submission_json_n(1) {
            panic!("Error getting sample submission json: {}", e)
        }
    }

    #[test]
    fn should_get_sample_action_receipts() {
        if let Err(e) = get_sample_action_receipts_n(1) {
            panic!("Error getting sample action receipts: {}", e)
        }
    }
}
