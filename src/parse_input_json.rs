use crate::{
    state::State,
    error::AppError,
    types::{
        Result,
        EosInputJson
    },
};

pub fn parse_eos_input_json_string(
    eos_input_json_string: &String
) -> Result<EosInputJson> {
    match serde_json::from_str(&eos_input_json_string) {
        Ok(result) => Ok(result),
        Err(e) => Err(AppError::Custom(e.to_string()))
    }
}

pub fn parse_input_json_string_and_put_in_state(state: State) -> Result<State> {
    trace!("✔ Parsing input json...");
    parse_eos_input_json_string(&state.cli_args.arg_JSON)
        .and_then(|eos_input_json| state.add_eos_input_json(eos_input_json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_utils::test_utils::get_sample_submission_string,
    };

    #[test]
    fn should_parse_input_json_string() {
        let expected_number_of_actions = 7;
        let expected_number_of_transactions = 4;
        let string = get_sample_submission_string()
            .unwrap();
        let result = parse_eos_input_json_string(&string)
            .unwrap();
        assert!(result.actions.len() == expected_number_of_actions);
        assert!(result.action_receipts.len() == expected_number_of_actions);
        assert!(result.block.transactions.len() == expected_number_of_transactions);
    }
}
