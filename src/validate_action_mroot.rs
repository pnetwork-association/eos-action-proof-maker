use crate::{
    state::State,
    error::AppError,
    eos_merkle_utils::get_merkle_digest,
    types::{
        Bytes,
        Result,
        EosActionReceipts,
    },
};

fn get_merkle_digest_from_action_receipts(
    action_receipts: &EosActionReceipts
) -> Bytes {
    get_merkle_digest(
        action_receipts
            .iter()
            .map(|receipt| receipt.to_digest())
            .collect()
    )
}

fn validate_merkle_digest(digest: &Bytes, action_mroot_hex: &String) -> Result<()> {
    match &hex::decode(action_mroot_hex)? == digest {
        true => Ok(()),
        false => Err(AppError::Custom(
            format!(
                "✘ Error validating action receipts!\n{}",
                "✘ Action receipt merkle root does NOT match `action_mroot`!"
            )
        ))
    }
}

pub fn validate_action_receipt_merkle_root(state: State) -> Result<State> {
    state
        .get_eos_action_receipts()
        .map(get_merkle_digest_from_action_receipts)
        .and_then(|digest|
             validate_merkle_digest(
                 &digest,
                 &state.get_eos_block()?.action_mroot
             )
        )
        .and_then(|_| Ok(state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::{
        get_sample_eos_block,
        get_sample_action_receipts,
    };

    #[test]
    fn should_get_merkle_digest_from_eos_action_receipts() {
        let expected_result = get_sample_eos_block()
            .unwrap()
            .action_mroot;
        let action_receipts = get_sample_action_receipts()
            .unwrap();
        let result = get_merkle_digest_from_action_receipts(&action_receipts);
        assert!(hex::encode(result) == expected_result);
    }

    #[test]
    fn should_validate_valid_merkle_digest() {
        let action_mroot_hex = get_sample_eos_block()
            .unwrap()
            .action_mroot;
        let action_receipts = get_sample_action_receipts()
            .unwrap();
        let valid_merkle_digests = get_merkle_digest_from_action_receipts(
            &action_receipts
        );
        if let Err(e) = validate_merkle_digest(
            &valid_merkle_digests,
            &action_mroot_hex,
        ) {
            panic!("Should validate valid merkle digest: {}", e);
        }
    }

    #[test]
    fn should_err_when_validating_invalid_merkle_digest() {
        let wrong_action_mroot_hex = get_sample_eos_block()
            .unwrap()
            .transaction_mroot;
        let action_receipts = get_sample_action_receipts()
            .unwrap();
        let valid_merkle_digests = get_merkle_digest_from_action_receipts(
            &action_receipts
        );
        if let Ok(_) = validate_merkle_digest(
            &valid_merkle_digests,
            &wrong_action_mroot_hex,
        ) {
            panic!("Should NOT validate invalid merkle digest!");
        }
    }
}
