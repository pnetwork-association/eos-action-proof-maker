use crate::{
    state::State,
    types::{
        Result,
        EosBlock,
        EosBlockJson,
    },
};

fn parse_block_json(block_json: &EosBlockJson) -> Result<EosBlock> {
    Ok(
        EosBlock {
            block_id: hex::decode(&block_json.block_id)?,
            previous: block_json.previous.clone(),
            producer: block_json.producer.clone(),
            //new_producers: serde_json::Value, // TODO: Handle! Could be null!
            confirmed: block_json.confirmed.clone(),
            schedule_version: block_json.schedule_version.clone(),
            //header_extensions: Vec<serde_json::Value>, // NOTE: Ibid!
            transaction_mroot: block_json.transaction_mroot.clone(),
            action_mroot: block_json.action_mroot.clone(),
        }
    )
}

pub fn parse_eos_block_and_put_in_state(state: State) -> Result<State> {
    trace!("✔ Parsing EOS block...");
    state
        .get_eos_input_json()
        .and_then(|json| parse_block_json(&json.block))
        .and_then(|eos_block| state.add_eos_block(eos_block))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::get_sample_submission_json;

    #[test]
    fn should_parse_eos_block_json() {
        let json = get_sample_submission_json()
            .unwrap();
        if let Err(e) = parse_block_json(&json.block) {
            panic!("Error parsing EOS block: {}", e)
        }
    }
}

