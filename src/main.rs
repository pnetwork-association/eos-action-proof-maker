pub mod state;
pub mod types;
pub mod error;
pub mod constants;
pub mod test_utils;
pub mod usage_info;
pub mod parse_cli_args;
pub mod parse_input_json;
pub mod parse_eos_actions;
pub mod initialize_logger;
pub mod parse_eos_action_receipts;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use crate::{
    initialize_logger::initialize_logger,
    parse_cli_args::parse_cli_args_and_put_in_state,
    parse_input_json::parse_input_json_string_and_put_in_state,
    parse_eos_actions::parse_eos_action_jsons_and_put_in_state,
    parse_eos_action_receipts::parse_eos_action_receipt_jsons_and_put_in_state,
};

fn main() {
    match parse_cli_args_and_put_in_state()
        .and_then(initialize_logger)
        .and_then(parse_input_json_string_and_put_in_state)
        .and_then(parse_eos_action_jsons_and_put_in_state)
        .and_then(parse_eos_action_receipt_jsons_and_put_in_state)
        {
            Ok(state) => {
                println!("{:?}", state)
            },
            Err(e) => {
                error!("{}", e);
                println!("{}", e);
                std::process::exit(1);
            }
        }
}
