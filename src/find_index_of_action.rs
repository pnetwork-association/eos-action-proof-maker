use crate::{
    error::AppError,
    get_action_digest::get_action_digest,
    state::State,
    types::{EosActionReceipts, Result},
};
use eos_chain::Action as EosAction;

fn get_index_of_action(action: &EosAction, action_receipts: &EosActionReceipts) -> Result<u32> {
    let mut index: Option<u32> = None;
    let sought_digest = get_action_digest(action)?;
    action_receipts
        .iter()
        .enumerate()
        .map(|(i, receipt)| {
            if receipt.act_digest.as_bytes() == sought_digest {
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

    #[cfg(all(test, feature = "disable-action-return-value-protocol-feature"))]
    fn should_get_index_of_action() {
        let sample_num = 1;
        let expected_result = 5;
        let action_receipts = get_sample_action_receipts_n(sample_num).unwrap();
        let action = get_sample_action_n(sample_num).unwrap();
        let result = get_index_of_action(&action, &action_receipts).unwrap();
        assert_eq!(result, expected_result);
    }
}
