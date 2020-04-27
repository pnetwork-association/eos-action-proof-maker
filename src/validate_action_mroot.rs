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

pub fn get_merkle_digest_from_action_receipts(
    action_receipts: &EosActionReceipts
) -> Bytes {
    get_merkle_digest(
        action_receipts
            .iter()
            .map(|receipt| receipt.to_digest())
            .collect()
    )
}

fn check_merkle_digest(
    digest: &Bytes,
    action_mroot_hex: &String
) -> Result<()> {
    debug!("Digest      : {}", hex::encode(digest));
    debug!("Action Mroot: {}", action_mroot_hex);
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
    info!("✔ Validating action-receipts merkle root...");
    state
        .get_eos_action_receipts()
        .map(|receipts| get_merkle_digest_from_action_receipts(&receipts))
        .and_then(|digest|
            check_merkle_digest(
                &digest,
                &state.get_eos_block()?.action_mroot
            )
        )
        .and_then(|_| Ok(state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_utils::{
            NUM_SAMPLES,
            get_sample_eos_block_n,
            get_sample_action_receipts_n,
        },
    };

    #[test]
    fn should_validate_digest_for_sample_blocks() {
        vec![0u8; NUM_SAMPLES]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let expected_result = get_sample_eos_block_n(i + 1)
                    .unwrap()
                    .action_mroot;
                let action_receipts = get_sample_action_receipts_n(i + 1)
                    .unwrap();
                let result = get_merkle_digest_from_action_receipts(
                    &action_receipts
                );
                assert_eq!(hex::encode(result), expected_result);
            })
            .for_each(drop);
    }

    #[test]
    fn should_validate_valid_merkle_digests() {
        vec![0u8; NUM_SAMPLES]
            .iter()
            .enumerate()
            .map(|(i, _)|{
                let action_mroot_hex = get_sample_eos_block_n(i + 1)
                    .unwrap()
                    .action_mroot;
                let action_receipts = get_sample_action_receipts_n(i + 1)
                    .unwrap();
                let valid_digests = get_merkle_digest_from_action_receipts(
                    &action_receipts
                );
                if let Err(e) = check_merkle_digest(
                    &valid_digests,
                    &action_mroot_hex,
                ) {
                    panic!("Should validate valid merkle digest: {}", e);
                }
            })
            .for_each(drop);
    }

    #[test]
    fn should_err_when_validating_invalid_merkle_digest() {
        let wrong_action_mroot_hex = get_sample_eos_block_n(1)
            .unwrap()
            .transaction_mroot;
        let action_receipts = get_sample_action_receipts_n(1)
            .unwrap();
        let valid_merkle_digests = get_merkle_digest_from_action_receipts(
            &action_receipts
        );
        if let Ok(_) = check_merkle_digest(
            &valid_merkle_digests,
            &wrong_action_mroot_hex,
        ) {
            panic!("Should NOT validate invalid merkle digest!");
        }
    }
}
