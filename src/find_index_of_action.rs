use crate::{
    error::AppError,
    get_action_digest::get_action_digest,
    state::State,
    types::{Byte, EosActionReceipts, Result},
};
use eos_chain::Action as EosAction;

// NOTE: There exists _first_ in every block a special action in the protocol itself...
const ON_BLOCK_ACTION_INDEX: u32 = 0;

fn get_index_of_action_digest(
    action_digest: &[Byte],
    action_receipts: &EosActionReceipts,
) -> Result<u32> {
    let index =
        action_receipts
            .iter()
            .enumerate()
            .fold(ON_BLOCK_ACTION_INDEX, |mut acc, (i, receipt)| {
                if receipt.act_digest.as_bytes() == action_digest {
                    acc = i as u32
                };
                acc
            });

    if index == ON_BLOCK_ACTION_INDEX {
        Err(AppError::Custom(
            "✘ Could not find action digest in action receipts!".to_string(),
        ))
    } else {
        Ok(index)
    }
}

fn get_index_of_action(action: &EosAction, action_receipts: &EosActionReceipts) -> Result<u32> {
    get_index_of_action_digest(&get_action_digest(action, false)?, action_receipts)
        .or_else(|_| get_index_of_action_digest(&get_action_digest(action, true)?, action_receipts))
}

pub fn find_index_of_action_and_put_in_state(state: State) -> Result<State> {
    info!("✔ Finding index of action in `action_receipts`...");
    get_index_of_action(state.get_eos_action()?, state.get_eos_action_receipts()?)
        .and_then(|index| state.add_proof_index(index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_sample_action_n, get_sample_action_receipts_n};

    #[cfg(all(test, feature = "disable-action-return-value-protocol-feature"))]
    fn should_get_index_of_action() {
        let sample_num = 1;
        let expected_result = 5;
        let action_receipts = get_sample_action_receipts_n(sample_num).unwrap();
        let action = get_sample_action_n(sample_num).unwrap();
        let result = get_index_of_action(&action, &action_receipts).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_index_of_action_2() {
        let sample_num = 2;
        let expected_result = 140;
        let action_receipts = get_sample_action_receipts_n(sample_num).unwrap();
        let action = get_sample_action_n(sample_num).unwrap();
        let result = get_index_of_action(&action, &action_receipts).unwrap();
        assert_eq!(result, expected_result);
    }
}
