pub mod state;
pub mod types;
pub mod error;
pub mod utils;
pub mod constants;
pub mod test_utils;
pub mod usage_info;
pub mod verify_proof;
pub mod generate_proof;
pub mod parse_cli_args;
pub mod generate_output;
pub mod parse_eos_block;
pub mod parse_input_json;
pub mod eos_merkle_utils;
pub mod parse_eos_action;
pub mod initialize_logger;
pub mod find_index_of_action;
pub mod validate_action_mroot;
pub mod parse_eos_action_receipts;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use crate::{
    types::Result,
    verify_proof::verify_proof_in_state,
    initialize_logger::initialize_logger,
    generate_output::generate_output_string,
    parse_cli_args::parse_cli_args_and_put_in_state,
    generate_proof::generate_proof_and_add_to_state,
    parse_eos_block::parse_eos_block_json_and_put_in_state,
    parse_eos_action::parse_eos_action_json_and_put_in_state,
    parse_input_json::parse_input_json_string_and_put_in_state,
    validate_action_mroot::validate_action_receipt_merkle_root,
    find_index_of_action::find_index_of_action_and_put_in_state,
    parse_eos_action_receipts::parse_eos_action_receipt_jsons_and_put_in_state,
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
