use std::str::FromStr;
use eos_primitives::{
    AccountName,
    Checksum256,
    AuthSequence,
    AuthSequences,
    ActionReceipt as EosActionReceipt,
    ActionReceipts as EosActionReceipts,
};
use crate::{
    state::State,
    types::{
        Result,
        AuthSequenceJson,
        AuthSequenceJsons,
        EosActionReceiptJson,
        EosActionReceiptJsons,
    },
};

pub fn convert_hex_to_checksum256(hex: &String) -> Result<Checksum256> {
    let mut arr = [0; 32];
    let bytes = hex::decode(hex)?;
    arr.copy_from_slice(&bytes);
    Ok(Checksum256::from(arr))
}

fn parse_auth_sequence_json(
    auth_sequence_json: &AuthSequenceJson
) -> Result<AuthSequence> {
    Ok(
        AuthSequence::new(
            &auth_sequence_json.0,
            auth_sequence_json.1
        )?
    )
}

fn parse_auth_sequence_jsons(
    auth_sequence_jsons: &AuthSequenceJsons
) -> Result<AuthSequences> {
    auth_sequence_jsons
        .iter()
        .map(parse_auth_sequence_json)
        .collect::<Result<AuthSequences>>()
}

fn parse_eos_action_receipt_json(
    eos_action_receipt_json: &EosActionReceiptJson
) -> Result<EosActionReceipt> {
    Ok(
        EosActionReceipt {
            abi_sequence: eos_action_receipt_json.abi_sequence,
            code_sequence: eos_action_receipt_json.code_sequence,
            recipient: AccountName::from_str(
                &eos_action_receipt_json
                    .receiver
            )?,
            act_digest: convert_hex_to_checksum256(
                &eos_action_receipt_json.act_digest
            )?,
            global_sequence: eos_action_receipt_json
                .global_sequence
                .into(),
            recv_sequence: eos_action_receipt_json
                .recv_sequence
                .into(),
            auth_sequence: parse_auth_sequence_jsons(
                &eos_action_receipt_json.auth_sequence
            )?,
        }
    )
}

pub fn parse_action_receipt_jsons(
    eos_action_receipt_jsons: &EosActionReceiptJsons
) -> Result<EosActionReceipts> {
    eos_action_receipt_jsons
        .into_iter()
        .map(parse_eos_action_receipt_json)
        .collect::<Result<EosActionReceipts>>()
}

pub fn parse_eos_action_receipt_jsons_and_put_in_state(state: State) -> Result<State> {
    trace!("✔ Parsing EOS actions...");
    state
        .get_eos_input_json()
        .and_then(|json| parse_action_receipt_jsons(&json.action_receipts))
        .and_then(|receipts| state.add_eos_action_receipts(receipts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::get_sample_submission_json;

    #[test]
    fn should_parse_action_receipt_jsons() {
        let expected_num_receipts = 7;
        let json = get_sample_submission_json()
            .unwrap();
        let result = parse_action_receipt_jsons(&json.action_receipts)
            .unwrap();
        assert!(result.len() == expected_num_receipts);
    }
}
