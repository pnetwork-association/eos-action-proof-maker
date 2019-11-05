use std::result;
use crate::{
    error::AppError
};
use eos_primitives::{
    Action as EosAction,
    ActionReceipt as EosActionReceipt,
};

pub type EosActions = Vec<EosAction>;
pub type EosActionJsons = Vec<EosActionJson>;
pub type Result<T> = result::Result<T, AppError>;
pub type EosActionReceipts = Vec<EosActionReceipt>;
pub type EosActionReceiptJsons = Vec<EosActionReceiptJson>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosInputJson {
    pub block: EosBlockJson,
    pub actions: EosActionJsons,
    pub action_receipts: EosActionReceiptJsons,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosBlockJson {
    pub block_id: String,
    pub block_num: u64,
    pub producer: String,
    pub confirmed: usize,
    pub previous: String,
    pub timestamp: String,
    pub action_mroot: String,
    pub schedule_version: usize,
    pub transaction_mroot: String,
    pub producer_signature: String,
    pub new_producers: serde_json::Value,
    pub transactions: Vec<serde_json::Value>, // TODO Real type for this!
    pub block_extensions: Vec<serde_json::Value>,
    pub header_extensions: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosActionJson {
    pub name: String,
    pub account: String,
    pub data: serde_json::Value, // NOTE: Could be hex string, or contract data!
    pub authorization: Vec<AuthorizationJson>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationJson {
    pub actor: String,
    pub permission: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosActionReceiptJson {
    pub receiver: String,
    pub act_digest: String,
    pub global_sequence: u64,
    pub recv_sequence:  u64,
    pub auth_sequence: Vec<(String, u64)>,
    pub code_sequence: usize,
    pub abi_sequence: usize,
}
