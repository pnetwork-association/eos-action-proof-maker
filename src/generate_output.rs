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
            &Output {
                tx_id: "".to_string(), // FIXME
                block_id:
                    hex::encode(&state.get_eos_block()?.block_id),
                action_index:
                    state.cli_args.arg_INDEX,
                action_proof:
                    state.get_merkle_proof()?.to_vec(),
                action_digest:
                    hex::encode(state.get_eos_action()?.to_digest()),
                serialized_action:
                    hex::encode(state.get_eos_action()?.serialize()),
                action_json:
                    state
                        .get_eos_input_json()?
                        .action
                        .clone(),
                action_receipt_json:
                    state
                        .get_eos_input_json()?
                        .action_receipts
                        [state.cli_args.arg_INDEX]
                        .clone(),
                action_receipt_digest:
                    hex::encode(
                        state
                            .get_eos_action_receipts()?
                            [state.cli_args.arg_INDEX]
                            .to_digest()
                    ),
                serialized_action_receipt:
                    hex::encode(
                        state
                            .get_eos_action_receipts()?
                            [state.cli_args.arg_INDEX]
                            .serialize()
                    ),
            }
        )?
    )
}
