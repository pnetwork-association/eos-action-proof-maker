use crate::{
    error::AppError,
    parse_cli_args::CliArgs,
    types::{EosActionReceipts, EosBlock, EosInputJson, MerkleProof, Result},
};
use eos_chain::Action as EosAction;

#[derive(Debug)]
pub struct State {
    pub cli_args: CliArgs,
    pub proof_index: Option<u32>,
    pub eos_block: Option<EosBlock>,
    pub eos_action: Option<EosAction>,
    pub merkle_proof: Option<MerkleProof>,
    pub eos_input_json: Option<EosInputJson>,
    pub eos_action_receipts: Option<EosActionReceipts>,
}

fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!", substring)
}

fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!", substring)
}

impl State {
    pub fn init(cli_args: CliArgs) -> Result<State> {
        Ok(State {
            cli_args,
            eos_block: None,
            eos_action: None,
            proof_index: None,
            merkle_proof: None,
            eos_input_json: None,
            eos_action_receipts: None,
        })
    }

    pub fn add_eos_input_json(mut self, eos_input_json: EosInputJson) -> Result<Self> {
        trace!("✔ Adding EOS input json to state!");
        match self.eos_input_json {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err(
                "eos_input_json",
            ))),
            None => {
                self.eos_input_json = Some(eos_input_json);
                Ok(self)
            }
        }
    }

    pub fn get_eos_input_json(&self) -> Result<&EosInputJson> {
        match &self.eos_input_json {
            Some(input_json) => Ok(input_json),
            None => Err(AppError::Custom(get_not_in_state_err("eos_input_json"))),
        }
    }

    pub fn add_eos_action(mut self, eos_action: EosAction) -> Result<Self> {
        trace!("✔ Adding EOS actions to state!");
        match self.eos_action {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("eos_action"))),
            None => {
                self.eos_action = Some(eos_action);
                Ok(self)
            }
        }
    }

    pub fn get_eos_action(&self) -> Result<&EosAction> {
        match &self.eos_action {
            Some(actions) => Ok(actions),
            None => Err(AppError::Custom(get_not_in_state_err("eos_action"))),
        }
    }

    pub fn add_eos_block(mut self, eos_block: EosBlock) -> Result<Self> {
        trace!("✔ Adding EOS actions to state!");
        match self.eos_block {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("eos_block"))),
            None => {
                self.eos_block = Some(eos_block);
                Ok(self)
            }
        }
    }

    pub fn get_eos_block(&self) -> Result<&EosBlock> {
        match &self.eos_block {
            Some(actions) => Ok(actions),
            None => Err(AppError::Custom(get_not_in_state_err("eos_block"))),
        }
    }

    pub fn add_merkle_proof(mut self, merkle_proof: MerkleProof) -> Result<Self> {
        trace!("✔ Adding mekle proof to state!");
        match self.merkle_proof {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("merkle_proof"))),
            None => {
                self.merkle_proof = Some(merkle_proof);
                Ok(self)
            }
        }
    }

    pub fn get_merkle_proof(&self) -> Result<&MerkleProof> {
        match &self.merkle_proof {
            Some(proof) => Ok(proof),
            None => Err(AppError::Custom(get_not_in_state_err("merkle_proof"))),
        }
    }

    pub fn add_proof_index(mut self, proof_index: u32) -> Result<Self> {
        trace!("✔ Adding mekle proof to state!");
        match self.proof_index {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err("proof_index"))),
            None => {
                self.proof_index = Some(proof_index);
                Ok(self)
            }
        }
    }

    pub fn get_proof_index(&self) -> Result<u32> {
        match &self.proof_index {
            Some(index) => Ok(*index),
            None => Err(AppError::Custom(get_not_in_state_err("proof_index"))),
        }
    }

    pub fn add_eos_action_receipts(
        mut self,
        eos_action_receipts: EosActionReceipts,
    ) -> Result<Self> {
        trace!("✔ Adding EOS receipts to state!");
        match self.eos_action_receipts {
            Some(_) => Err(AppError::Custom(get_no_overwrite_state_err(
                "eos_action_receipts",
            ))),
            None => {
                self.eos_action_receipts = Some(eos_action_receipts);
                Ok(self)
            }
        }
    }

    pub fn get_eos_action_receipts(&self) -> Result<&EosActionReceipts> {
        match &self.eos_action_receipts {
            Some(receipts) => Ok(receipts),
            None => Err(AppError::Custom(get_not_in_state_err(
                "eos_action_receipts",
            ))),
        }
    }
}
