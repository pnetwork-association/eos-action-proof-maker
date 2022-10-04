use crate::{
    action_return_values::action_return_value_protocol_feature_is_enabled,
    get_action_digest::get_action_digest,
    state::State,
    types::{Output, Result},
};
use eos_chain::{Digest, SerializeData};
use serde_json;

pub fn generate_output_string(state: State) -> Result<String> {
    let action = state.get_eos_action()?;
    let action_receipts = state.get_eos_action_receipts()?;

    let action_return_value_is_enabled =
        action_return_value_protocol_feature_is_enabled(action, action_receipts)?;

    Ok(serde_json::to_string(&Output {
        tx_id: state.get_eos_input_json()?.action_receipts[state.get_proof_index()? as usize]
            .tx_id
            .clone(),
        block_id: hex::encode(&state.get_eos_block()?.block_id),
        action_index: state.get_proof_index()? as usize,
        action_proof: state.get_merkle_proof()?.to_vec(),
        action_digest: format!(
            "0x{}",
            hex::encode(get_action_digest(action, action_return_value_is_enabled)?)
        ),
        serialized_action: hex::encode(action.to_serialize_data()?),
        action_json: state.get_eos_input_json()?.action.clone(),
        action_receipt_json: state.get_eos_input_json()?.action_receipts
            [state.get_proof_index()? as usize]
            .clone(),
        action_receipt_digest: format!(
            "0x{}",
            action_receipts[state.get_proof_index()? as usize].digest()?,
        ),
        serialized_action_receipt: hex::encode(
            action_receipts[state.get_proof_index()? as usize].to_serialize_data()?,
        ),
    })?)
}
