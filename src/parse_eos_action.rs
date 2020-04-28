use std::str::FromStr;
use crate::{
    state::State,
    error::AppError,
    types::{
        Bytes,
        Result,
        EosActionJson,
        AuthorizationJson,
        AuthorizationJsons,
    },
};
use eos_primitives::{
    ActionName,
    AccountName,
    PermissionLevel,
    PermissionLevels,
    Action as EosAction,
};

fn parse_authorization_json(
    authorization_json: &AuthorizationJson
) -> Result<PermissionLevel> {
    Ok(
        PermissionLevel::from_str(
            authorization_json.actor.clone(),
            authorization_json.permission.clone(),
        )?
    )
}

fn parse_authorization_jsons(
    authorization_jsons: &AuthorizationJsons
) -> Result<PermissionLevels> {
    authorization_jsons
        .iter()
        .map(parse_authorization_json)
        .collect::<Result<PermissionLevels>>()
}

fn deserialize_action_data(
    action_data: &serde_json::Value,
    maybe_hex_data: &Option<String>,
) -> Result<Bytes> {
    match action_data {
        serde_json::Value::String(string) => Ok(hex::decode(string)?),
        serde_json::Value::Object(_) => match maybe_hex_data {
            Some(string) => Ok(hex::decode(string)?),
            None => Err(AppError::Custom(
                "✘ Failed to decode hex_data field of action!".to_string()
            ))
        }
        _ => Err(AppError::Custom(
            "✘ Failed to decode data field of action!".to_string()
        ))
    }
}

pub fn parse_eos_action_json(action_json: &EosActionJson) -> Result<EosAction> {
    Ok(
        EosAction {
            account: AccountName::from_str(
                &action_json.account
            )?,
            name: ActionName::from_str(
                &action_json.name
            )?,
            authorization: parse_authorization_jsons(
                &action_json.authorization
            )?,
            data: deserialize_action_data(
                &action_json.data,
                &action_json.hex_data,
            )?,
        }
    )
}

pub fn parse_eos_action_json_and_put_in_state(state: State) -> Result<State> {
    trace!("✔ Parsing EOS action...");
    state
        .get_eos_input_json()
        .and_then(|json| parse_eos_action_json(&json.action))
        .and_then(|action| state.add_eos_action(action))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_sample_submission_json_n;

    #[test]
    fn should_parse_eos_action_json() {
        let action_json = get_sample_submission_json_n(1)
            .unwrap()
            .action;
        if let Err(e) = parse_eos_action_json(&action_json) {
            panic!("Error parsing eos actions: {}", e);
        }
    }
}
