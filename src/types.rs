use std::result;
use crate::{
    error::AppError
};
use eos_primitives::{
    Action as EosAction,
    ActionReceipt as EosActionReceipt,
};

pub type Byte = u8;
pub type Bytes = Vec<Byte>;
pub type MerkleProof = Vec<String>;
pub type ActionProof = MerkleProof;
pub type EosActions = Vec<EosAction>;
pub type Result<T> = result::Result<T, AppError>;
pub type EosActionReceipts = Vec<EosActionReceipt>;
pub type AuthSequenceJsons = Vec<AuthSequenceJson>;
pub type AuthorizationJsons = Vec<AuthorizationJson>;
pub type EosTransactionJsons = Vec<EosTransactionJson>;
pub type EosActionReceiptJsons = Vec<EosActionReceiptJson>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub tx_id: String,
    pub block_id: String,
    pub action_index: usize,
    pub action_digest: String,
    pub action_proof: MerkleProof,
    pub serialized_action: String,
    pub action_json: EosActionJson,
    pub action_receipt_digest: String,
    pub serialized_action_receipt: String,
    pub action_receipt_json: EosActionReceiptJson,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosInputJson {
    pub block: EosBlockJson,
    pub action: EosActionJson,
    pub action_receipts: EosActionReceiptJsons,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosTransactionJson {
    pub id: String,
    pub action_traces: ActionTraceJsons,
}

pub type ActionTraceJsons = Vec<ActionTraceJson>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionTraceJson {
    pub receipt: EosActionReceiptJson,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosBlockJson {
    pub block_id: String,
    pub timestamp: String,
    pub producer: String,
    pub confirmed: usize,
    pub previous: String,
    pub action_mroot: String,
    pub transaction_mroot: String,
    pub schedule_version: usize,
    pub new_producers: serde_json::Value,
    pub header_extensions: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosBlock {
    pub block_id: Bytes,
    pub previous: String,
    pub producer: String,
    pub confirmed: usize,
    pub action_mroot: String,
    pub schedule_version: usize,
    pub transaction_mroot: String,
    pub header_extensions: Option<String>,
    pub new_producers: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosActionJson {
    pub name: String,
    pub account: String,
    pub data: serde_json::Value, // NOTE: Could be hex string, or contract data!
    pub hex_data: Option<String>,
    pub authorization: AuthorizationJsons,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationJson {
    pub actor: String,
    pub permission: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthSequenceJson(pub String, pub u64);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosActionReceiptJson {
    pub tx_id: String,
    pub receiver: String,
    pub act_digest: String,
    pub global_sequence: u64,
    pub recv_sequence:  u64,
    pub auth_sequence: AuthSequenceJsons,
    pub code_sequence: usize,
    pub abi_sequence: usize,
}
