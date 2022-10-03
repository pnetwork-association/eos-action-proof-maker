use crate::{
    get_action_digest::get_action_digest,
    state::State,
    types::{Output, Result},
};
use eos_chain::{Digest, SerializeData};
use serde_json;

pub fn generate_output_string(state: State) -> Result<String> {
    Ok(serde_json::to_string(&Output {
        tx_id: state.get_eos_input_json()?.action_receipts[state.get_proof_index()? as usize]
            .tx_id
            .clone(),
        block_id: hex::encode(&state.get_eos_block()?.block_id),
        action_index: state.get_proof_index()? as usize,
        action_proof: state.get_merkle_proof()?.to_vec(),
        action_digest: format!(
            "0x{}",
            hex::encode(get_action_digest(state.get_eos_action()?)?)
        ),
        serialized_action: hex::encode(state.get_eos_action()?.to_serialize_data()?),
        action_json: state.get_eos_input_json()?.action.clone(),
        action_receipt_json: state.get_eos_input_json()?.action_receipts
            [state.get_proof_index()? as usize]
            .clone(),
        action_receipt_digest: format!(
            "0x{}",
            state.get_eos_action_receipts()?[state.get_proof_index()? as usize].digest()?,
        ),
        serialized_action_receipt: hex::encode(
            state.get_eos_action_receipts()?[state.get_proof_index()? as usize]
                .to_serialize_data()?,
        ),
    })?)
}
