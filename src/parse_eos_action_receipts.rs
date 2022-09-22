use crate::{
    state::State,
    types::{
        AuthSequenceJson, AuthSequenceJsons, EosActionReceiptJson, EosActionReceiptJsons, Result,
    },
};
use eos_chain::{
    utils::flat_map::FlatMap, AccountName, ActionReceipt as EosActionReceipt, Checksum256,
    UnsignedInt,
};
use std::str::FromStr;

pub fn sort_action_receipts_by_global_sequence(
    action_receipts: Vec<EosActionReceipt>,
) -> Vec<EosActionReceipt> {
    let mut sorted = action_receipts;
    sorted.sort_by(|a, b| a.global_sequence.cmp(&b.global_sequence));
    sorted
}

pub fn convert_hex_to_checksum256(hex: &String) -> Result<Checksum256> {
    let mut arr = [0; 32];
    let bytes = hex::decode(hex)?;
    arr.copy_from_slice(&bytes);
    Ok(Checksum256::from(arr))
}

fn parse_auth_sequence_json(auth_sequence_json: &AuthSequenceJson) -> Result<(AccountName, u64)> {
    Ok((
        AccountName::from_str(&auth_sequence_json.0)?,
        auth_sequence_json.1,
    ))
}

fn parse_auth_sequence_jsons(
    auth_sequence_jsons: &AuthSequenceJsons,
) -> Result<FlatMap<AccountName, u64>> {
    Ok(FlatMap::assign(
        auth_sequence_jsons
            .iter()
            .map(parse_auth_sequence_json)
            .collect::<Result<Vec<(AccountName, u64)>>>()?,
    ))
}

fn parse_eos_action_receipt_json(
    eos_action_receipt_json: &EosActionReceiptJson,
) -> Result<EosActionReceipt> {
    Ok(EosActionReceipt {
        abi_sequence: UnsignedInt::from(eos_action_receipt_json.abi_sequence),
        code_sequence: UnsignedInt::from(eos_action_receipt_json.code_sequence),
        receiver: AccountName::from_str(&eos_action_receipt_json.receiver)?,
        act_digest: convert_hex_to_checksum256(&eos_action_receipt_json.act_digest)?,
        global_sequence: eos_action_receipt_json.global_sequence,
        recv_sequence: eos_action_receipt_json.recv_sequence,
        auth_sequence: parse_auth_sequence_jsons(&eos_action_receipt_json.auth_sequence)?,
    })
}

pub fn parse_action_receipt_jsons(
    eos_action_receipt_jsons: &EosActionReceiptJsons,
) -> Result<Vec<EosActionReceipt>> {
    eos_action_receipt_jsons
        .iter()
        .map(parse_eos_action_receipt_json)
        .collect::<Result<Vec<EosActionReceipt>>>()
}

pub fn parse_eos_action_receipt_jsons_and_put_in_state(state: State) -> Result<State> {
    trace!("✔ Parsing EOS action receipts...");
    parse_action_receipt_jsons(&state.get_eos_input_json()?.action_receipts)
        .map(sort_action_receipts_by_global_sequence)
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
        let expected_num_receipts = 6;
        let json = get_sample_submission_json_n(1).unwrap();
        let result = parse_action_receipt_jsons(&json.action_receipts).unwrap();
        assert_eq!(result.len(), expected_num_receipts);
    }

    #[test]
    fn should_sort_action_receipts_by_global_sequence() {
        let expected_result_before_sort =
            "7cc717a7e256683ab4d01c05040fc503f2436625f5ac9f639a2fd0b201231564";
        let action_receipts = get_sample_submission_json_n(1)
            .and_then(|json| parse_action_receipt_jsons(&json.action_receipts))
            .unwrap();
        let digest_before_sorting =
            hex::encode(get_merkle_digest_from_action_receipts(&action_receipts).unwrap());
        assert_ne!(digest_before_sorting, expected_result_before_sort);
        let sorted_action_receipts = sort_action_receipts_by_global_sequence(action_receipts);
        let digest_after_sorting =
            hex::encode(get_merkle_digest_from_action_receipts(&sorted_action_receipts).unwrap());
        assert_eq!(digest_after_sorting, expected_result_before_sort);
        assert_ne!(digest_after_sorting, digest_before_sorting);
    }
}
