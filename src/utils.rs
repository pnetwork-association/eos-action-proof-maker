use crate::{
    error::AppError,
    types::{Bytes, Result},
};
use eos_chain::Checksum256;

pub fn convert_bytes_to_checksum256(bytes: &Bytes) -> Result<Checksum256> {
    match bytes.len() {
        32 => {
            let mut arr = [0; 32];
            arr.copy_from_slice(bytes);
            Ok(Checksum256::from(arr))
        }
        _ => Err(AppError::Custom(format!(
            "✘ Wrong number of bytes. Expected 32, got {}",
            bytes.len()
        ))),
    }
}
