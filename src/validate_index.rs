use crate::{
    state::State,
    types::Result,
    error::AppError,
};

fn validate_index(index: &usize, num_action_receipts: &usize) -> Result<()> {
    match &(index + 1) <= num_action_receipts {
        true => Ok(()),
        false => Err(AppError::Custom(
            format!(
                "✘ Cannot create proof for action at index {}!\n{}",
                index,
                "✘ Not enough action receipts supplied!",
            )
        ))
    }
}

pub fn validate_index_is_in_range(state: State) -> Result<State> {
    state
        .get_eos_action_receipts()
        .and_then(|receipts|
            validate_index(
                &state.cli_args.arg_INDEX,
                &receipts.len()
            )
        )
        .and_then(|_| Ok(state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_if_index_lte_num_receipts() {
        let index = 5;
        let num_receipts = 6;
        assert!(index <= num_receipts);
        if let Err(e) = validate_index(&index, &num_receipts) {
            panic!("Should not error validating valid index: {}", e);
        }
    }

    #[test]
    fn should_err_if_index_gt_num_receipts() {
        let index = 7;
        let num_receipts = 6;
        assert!(index > num_receipts);
        if let Ok(_) = validate_index(&index, &num_receipts) {
            panic!("Should error validating invalid index!");
        }
    }
}
