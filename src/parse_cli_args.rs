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
    pub arg_INDEX: usize,
    pub flag_file: String,
    pub cmd_generate_proof: bool,
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
            Err(e) => Err(AppError::Custom(e.to_string()))
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

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::get_sample_cli_args;

    #[test]
    fn should_put_cli_args_in_state() {
        let sample_cli_args = get_sample_cli_args(true, true, true);
        let result = put_cli_args_in_state(sample_cli_args.clone())
            .unwrap();
        assert!(result.cli_args == sample_cli_args)
    }

    #[test]
    fn should_update_block_in_cli_args() {
        let sample_cli_args = get_sample_cli_args(true, true, true);
        let new_block = "{dummy_block}".to_string();
        let result = sample_cli_args
            .update_json_in_cli_args(new_block.clone())
            .unwrap();
        assert!(result.arg_JSON.contains(&new_block));
    }

    #[test]
    fn should_read_from_file_and_update_cli_args_block_if_flag_set() {
        use std::fs::File;
        use std::io::prelude::*;
        use std::fs::remove_file;
        let path = "./temp-test-file-to-delete".to_string();
        let block_before = "block_before".to_string();
        let block_after = "block_after";
        let cli_args = CliArgs {
            cmd_initialize: false,
            flag_file: path.clone(),
            cmd_submitEthBlock: false,
            cmd_submitEosBlock: false,
            cmd_getBlockNumbers: false,
            cmd_reportEnclaveState: false,
            arg_JSON: block_before.clone(),
        };
        let mut file = File::create(path.clone()).unwrap();
        file.write_all(b"block_after").unwrap();
        let result = maybe_read_block_json_from_file(cli_args)
            .unwrap();
        remove_file(path.clone()).unwrap();
        assert!(!Path::new(&path).exists());
        assert!(result.arg_JSON == block_after);
        assert!(result.arg_JSON != block_before);
    }
}
*/
