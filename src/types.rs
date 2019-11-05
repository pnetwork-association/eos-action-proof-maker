use std::result;
use crate::{
    error::AppError
};
use eos_primitives::{
    Action as EosAction,
    ActionReceipt as EosActionReceipt,
};

pub type EosActions = Vec<EosAction>;
pub type Result<T> = result::Result<T, AppError>;
pub type EosActionReceipts = Vec<EosActionReceipt>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosInputJson {
    pub eos_block: String, // TODO When we know the type better!
    pub eos_actions: Vec<EosActionJson>,
    pub eos_action_receipts: Vec<EosActionReceiptJson>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosActionJson {
    pub data: String,
    pub name: String,
    pub account: String,
    pub raw_data: String,
    pub authorization: Vec<(String, u64)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosActionReceiptJson {
    pub receiver: String,
    pub act_digest: String,
    pub global_sequence: u64,
    pub recv_sequence: u64,
    pub auth_sequence: (String, u64), // FIXME: Can serde_json deserialize a tuple?
    pub code_sequence: usize,
    pub abi_sequence: usize,
}
