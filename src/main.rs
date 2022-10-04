mod action_return_values;
mod constants;
mod eos_merkle_utils;
mod error;
mod find_index_of_action;
mod generate_output;
mod generate_proof;
mod get_action_digest;
mod initialize_logger;
mod parse_cli_args;
mod parse_eos_action;
mod parse_eos_action_receipts;
mod parse_eos_block;
mod parse_input_json;
mod state;
mod test_utils;
mod types;
mod usage_info;
mod validate_action_mroot;
mod verify_proof;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use crate::{
    find_index_of_action::find_index_of_action_and_put_in_state,
    generate_output::generate_output_string, generate_proof::generate_proof_and_add_to_state,
    initialize_logger::initialize_logger, parse_cli_args::parse_cli_args_and_put_in_state,
    parse_eos_action::parse_eos_action_json_and_put_in_state,
    parse_eos_action_receipts::parse_eos_action_receipt_jsons_and_put_in_state,
    parse_eos_block::parse_eos_block_json_and_put_in_state,
    parse_input_json::parse_input_json_string_and_put_in_state, types::Result,
    validate_action_mroot::validate_action_receipt_merkle_root,
    verify_proof::verify_proof_in_state,
};

fn main() -> Result<()> {
    match parse_cli_args_and_put_in_state()
        .and_then(initialize_logger)
        .and_then(parse_input_json_string_and_put_in_state)
        .and_then(parse_eos_block_json_and_put_in_state)
        .and_then(parse_eos_action_json_and_put_in_state)
        .and_then(parse_eos_action_receipt_jsons_and_put_in_state)
        .and_then(find_index_of_action_and_put_in_state)
        .and_then(validate_action_receipt_merkle_root)
        .and_then(generate_proof_and_add_to_state)
        .and_then(verify_proof_in_state)
        .and_then(generate_output_string)
    {
        Ok(output) => {
            trace!("{}", output);
            println!("{}", output);
            Ok(())
        }
        Err(e) => {
            error!("{}", e);
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
