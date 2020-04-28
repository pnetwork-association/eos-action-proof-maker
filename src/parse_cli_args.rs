use docopt::Docopt;
use std::{
    path::Path,
    fs::read_to_string,
};
use crate::{
    state::State,
    types::Result,
    error::AppError,
    usage_info::USAGE_INFO,
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CliArgs {
    pub arg_JSON: String,
    pub flag_file: String,
    pub cmd_generate: bool,
}

impl CliArgs {
    pub fn update_json_in_cli_args(
        mut self,
        block_json: String
    ) -> Result<Self> {
        self.arg_JSON = block_json;
        Ok(self)
    }
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO)
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(_) => Err(AppError::Custom(USAGE_INFO.to_string()))
        }
}

pub fn maybe_read_block_json_from_file(
    cli_args: CliArgs
) -> Result<CliArgs> {
    match Path::new(&cli_args.flag_file).exists() {
        true => {
            info!(
                "✔ File exists @ path: {},\n✔ Reading file...",
                cli_args.flag_file,
            );
            cli_args
                .clone()
                .update_json_in_cli_args(read_to_string(cli_args.flag_file)?)
        }
        false => {
            info!(
                "✔ No file exists @ path: {}\n✔ Not reading file...",
                cli_args.flag_file,
            );
            Ok(cli_args)
        }
    }
}

pub fn put_cli_args_in_state(cli_args: CliArgs) -> Result<State> {
    State::init(cli_args)
}

pub fn parse_cli_args_and_put_in_state() -> Result<State> {
    parse_cli_args()
        .and_then(maybe_read_block_json_from_file)
        .and_then(put_cli_args_in_state)
}
