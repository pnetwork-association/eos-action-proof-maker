use crate::{
    error::AppError,
    parse_cli_args::CliArgs,
    types::{
        Result,
        EosBlock,
        EosActions,
        EosInputJson,
        EosActionReceipts,
    },
};

#[derive(Debug)]
pub struct State {
    pub cli_args: CliArgs,
    pub eos_block: Option<EosBlock>,
    pub eos_actions: Option<EosActions>,
    pub eos_input_json: Option<EosInputJson>,
    pub eos_action_receipts: Option<EosActionReceipts>,
}

fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!" , substring)
}

fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!" , substring)
}

impl State {
    pub fn init(
        cli_args: CliArgs
    ) -> Result<State> {
        Ok(
            State {
                cli_args,
                eos_block: None,
                eos_actions: None,
                eos_input_json: None,
                eos_action_receipts: None,
            }
        )
    }

    pub fn add_eos_input_json(
        mut self,
        eos_input_json: EosInputJson
    ) -> Result<Self> {
        trace!("✔ Adding EOS input json to state!");
        match self.eos_input_json {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("eos_input_json"))
            ),
            None => {
                self.eos_input_json = Some(eos_input_json);
                Ok(self)
            }
        }
    }

    pub fn get_eos_input_json(&self) -> Result<&EosInputJson> {
        match &self.eos_input_json {
            Some(input_json) => Ok(input_json),
            None => Err(AppError::Custom(
                get_not_in_state_err("eos_input_json")
            ))
        }
    }

    pub fn add_eos_actions(mut self, eos_actions: EosActions) -> Result<Self> {
        trace!("✔ Adding EOS actions to state!");
        match self.eos_actions {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("eos_actions")
            )),
            None => {
                self.eos_actions = Some(eos_actions);
                Ok(self)
            }
        }
    }

    pub fn get_eos_actions(&self) -> Result<&EosActions> {
        match &self.eos_actions {
            Some(actions) => Ok(actions),
            None => Err(AppError::Custom(
                get_not_in_state_err("eos_actions")
            ))
        }
    }

    pub fn add_eos_block(mut self, eos_block: EosBlock) -> Result<Self> {
        trace!("✔ Adding EOS actions to state!");
        match self.eos_block {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("eos_block")
            )),
            None => {
                self.eos_block = Some(eos_block);
                Ok(self)
            }
        }
    }

    pub fn get_eos_block(&self) -> Result<&EosBlock> {
        match &self.eos_block {
            Some(actions) => Ok(actions),
            None => Err(AppError::Custom(
                get_not_in_state_err("eos_block")
            ))
        }
    }

    pub fn add_eos_action_receipts(
        mut self,
        eos_action_receipts: EosActionReceipts
    ) -> Result<Self> {
        trace!("✔ Adding EOS receipts to state!");
        match self.eos_action_receipts {
            Some(_) => Err(AppError::Custom(
                get_no_overwrite_state_err("eos_action_receipts")
            )),
            None => {
                self.eos_action_receipts = Some(eos_action_receipts);
                Ok(self)
            }
        }
    }

    pub fn get_eos_action_receipts(&self) -> Result<&EosActionReceipts> {
        match &self.eos_action_receipts {
            Some(receipts) => Ok(receipts),
            None => Err(AppError::Custom(
                get_not_in_state_err("eos_action_receipts")
            ))
        }
    }
}
