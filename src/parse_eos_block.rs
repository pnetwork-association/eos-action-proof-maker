use crate::{
    state::State,
    types::{
        Result,
        EosBlock,
        EosBlockJson,
    },
};

pub fn parse_eos_block_json(block_json: &EosBlockJson) -> Result<EosBlock> {
    Ok(
        EosBlock {
            confirmed: block_json.confirmed,
            previous: block_json.previous.clone(),
            producer: block_json.producer.clone(),
            new_producers: serde_json::Value::Null,
            block_id: hex::decode(&block_json.block_id)?,
            action_mroot: block_json.action_mroot.clone(),
            schedule_version: block_json.schedule_version,
            header_extensions: block_json.header_extensions.clone(),
            transaction_mroot: block_json.transaction_mroot.clone(),
        }
    )
}

pub fn parse_eos_block_json_and_put_in_state(state: State) -> Result<State> {
    trace!("✔ Parsing EOS block json...");
    state
        .get_eos_input_json()
        .and_then(|json| parse_eos_block_json(&json.block))
        .and_then(|eos_block| state.add_eos_block(eos_block))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_sample_submission_json_n;

    #[test]
    fn should_parse_eos_block_json() {
        let json = get_sample_submission_json_n(1)
            .unwrap();
        if let Err(e) = parse_eos_block_json(&json.block) {
            panic!("Error parsing EOS block: {}", e)
        }
    }
}

