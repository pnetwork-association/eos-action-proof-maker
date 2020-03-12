use serde_json;
use crate::{
    state::State,
    types::{
        Result,
        Output,
    },
};

pub fn generate_output_string(state: State) -> Result<String> {
    Ok(
        serde_json::to_string(
            &Output::new(
                state.get_eos_actions_with_id()?[
                    state.cli_args.arg_INDEX
                ].tx_id.clone(),
                hex::encode(&state.get_eos_block()?.block_id),
                state.get_eos_actions()?[state.cli_args.arg_INDEX].clone(),
                state.cli_args.arg_INDEX,
                state.get_merkle_proof()?.to_vec(),
                hex::encode(
                    state
                        .get_eos_actions()?[state.cli_args.arg_INDEX]
                        .to_digest()
                ),
                hex::encode(
                    state
                        .get_eos_actions()?[state.cli_args.arg_INDEX]
                        .serialize()
                ),
                hex::encode(
                    state
                        .get_eos_action_receipts()?[state.cli_args.arg_INDEX]
                        .to_digest()
                ),
                hex::encode(
                    state
                        .get_eos_action_receipts()?[state.cli_args.arg_INDEX]
                        .serialize()
                ),
            )
        )?
    )
}
