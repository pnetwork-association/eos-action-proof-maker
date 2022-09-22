use crate::{
    error::AppError,
    state::State,
    types::{EosActionReceipts, Result},
    utils::convert_bytes_to_checksum256,
};
use eos_primitives::{Action as EosAction, Checksum256};

fn get_digest_from_action(action: &EosAction) -> Result<Checksum256> {
    convert_bytes_to_checksum256(&action.to_digest())
}

fn get_index_of_action(action: &EosAction, action_receipts: &EosActionReceipts) -> Result<u32> {
    let mut index: Option<u32> = None;
    let sought_digest = get_digest_from_action(action)?;
    action_receipts
        .iter()
        .enumerate()
        .map(|(i, receipt)| {
            if receipt.act_digest == sought_digest {
                index = Some(i as u32)
            }
        })
        .for_each(drop);
    match index {
        Some(idx) => Ok(idx),
        None => Err(AppError::Custom(
            "✘ Could not find action digest in action receipts!".to_string(),
        )),
    }
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

    #[test]
    fn should_get_digest_from_action() {
        let expected_result =
            "364afa1cc13bca5dce1027f089e56889171373f66f5e3e59637251aaaeac4caa".to_string();
        let action = get_sample_action_n(1).unwrap();
        let result = get_digest_from_action(&action).unwrap();
        assert_eq!(result.to_string(), expected_result)
    }

    #[test]
    fn should_get_index_of_action() {
        let sample_num = 1;
        let expected_result = 5;
        let action_receipts = get_sample_action_receipts_n(sample_num).unwrap();
        let action = get_sample_action_n(sample_num).unwrap();
        let result = get_index_of_action(&action, &action_receipts).unwrap();
        assert_eq!(result, expected_result);
    }
}
