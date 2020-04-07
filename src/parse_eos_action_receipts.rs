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
        EosActionReceiptAndIdJson,
    },
};

pub fn sort_action_receipts_by_global_sequence(
    action_receipts: &EosActionReceipts
) -> EosActionReceipts {
    let mut sorted = action_receipts.clone();
    sorted.sort_by(|a, b| a.global_sequence.cmp(&b.global_sequence));
    sorted
}

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

fn get_actions_jsons_from_actions_with_ids(
    actions_with_ids: &Vec<EosActionReceiptAndIdJson>,
) -> Result<EosActionReceiptJsons> {
    Ok(
        actions_with_ids
            .iter()
            .map(|action_with_id| action_with_id.action_receipt_json.clone())
            .collect::<EosActionReceiptJsons>()
    )
}

pub fn parse_eos_action_receipt_jsons_and_put_in_state(
    state: State
) -> Result<State> {
    trace!("✔ Parsing EOS actions...");
    state
        .get_eos_actions_with_id()
        .and_then(|actions_with_ids|
            get_actions_jsons_from_actions_with_ids(actions_with_ids)
        )
        .and_then(|receipt_jsons| parse_action_receipt_jsons(&receipt_jsons))
        .map(|receipts| sort_action_receipts_by_global_sequence(&receipts))
        .and_then(|receipts| state.add_eos_action_receipts(receipts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_utils::get_sample_submission_json_n,
        validate_action_mroot::get_merkle_digest_from_action_receipts,
    };

    #[test]
    fn should_parse_action_receipt_jsons() {
        let expected_num_receipts = 4;
        let json = get_sample_submission_json_n(1)
            .unwrap();
        let result = parse_action_receipt_jsons(&json.action_receipts)
            .unwrap();
        assert!(result.len() == expected_num_receipts);
    }

    #[test]
    fn should_sort_action_receipts_by_global_sequence() {
        let expected_result =
            "7cc717a7e256683ab4d01c05040fc503f2436625f5ac9f639a2fd0b201231564";
        let action_receipts = get_sample_submission_json_n(6)
            .and_then(|json| parse_action_receipt_jsons(&json.action_receipts))
            .unwrap();
        let digest_before_sorting = get_merkle_digest_from_action_receipts(
            &action_receipts
        );
        assert_ne!(hex::encode(digest_before_sorting), expected_result);
        let sorted_action_receipts = sort_action_receipts_by_global_sequence(
            &action_receipts
        );
        let digest_after_sorting = get_merkle_digest_from_action_receipts(
            &sorted_action_receipts
        );
        assert_eq!(hex::encode(digest_after_sorting), expected_result);
    }
}
