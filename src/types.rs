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
pub type EosActions = Vec<EosAction>;
pub type EosActionJsons = Vec<EosActionJson>;
pub type Result<T> = result::Result<T, AppError>;
pub type EosActionReceipts = Vec<EosActionReceipt>;
pub type AuthSequenceJsons = Vec<AuthSequenceJson>;
pub type AuthorizationJsons = Vec<AuthorizationJson>;
pub type EosActionReceiptJsons = Vec<EosActionReceiptJson>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub block_id: String,
    pub action_index: usize,
    pub merkle_proof: MerkleProof,
    pub action_digest: String,
    pub serialized_action: String,
    pub action_receipt_digest: String,
    pub serialized_action_receipt: String,
}

impl Output {
    pub fn new(
        block_id: String,
        action_index: usize,
        merkle_proof: MerkleProof,
        action_digest: String,
        serialized_action: String,
        action_receipt_digest: String,
        serialized_action_receipt: String,
    ) -> Self {
        Output {
            block_id,
            action_index,
            merkle_proof,
            action_digest,
            serialized_action,
            action_receipt_digest,
            serialized_action_receipt,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosInputJson {
    pub block: EosBlockJson,
    pub actions: EosActionJsons,
    pub action_receipts: EosActionReceiptJsons,
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
    pub header_extensions: Option<String>,//Vec<serde_json::Value>,
    //pub block_num: u64,
    //pub producer_signature: String,
    //pub transactions: Vec<serde_json::Value>, // TODO Real type for this!
    //pub block_extensions: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EosBlock {
    pub block_id: Bytes,
    pub previous: String,
    pub producer: String,
    pub new_producers: serde_json::Value, // TODO: Handle! Could be null!
    pub confirmed: usize,
    pub schedule_version: usize,
    pub header_extensions: Option<String>,// Vec<serde_json::Value>, // TODO: Handle! Could be null!
    pub transaction_mroot: String,
    pub action_mroot: String,
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
    pub receiver: String,
    pub act_digest: String,
    pub global_sequence: u64,
    pub recv_sequence:  u64,
    pub auth_sequence: AuthSequenceJsons,
    pub code_sequence: usize,
    pub abi_sequence: usize,
}
