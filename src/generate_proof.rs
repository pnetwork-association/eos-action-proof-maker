use crate::{
    state::State,
    eos_merkle_utils::generate_merkle_proof,
    types::{
        Result,
        MerkleProof,
        EosActionReceipts,
    },
};

fn generate_merkle_proof_from_action_receipts(
    index: &usize,
    action_receipts: &EosActionReceipts,
) -> Result<MerkleProof> {
    generate_merkle_proof(
        index.clone(),
        action_receipts
            .iter()
            .map(|receipt| receipt.to_digest())
            .collect()
    )
}

pub fn generate_proof_and_add_to_state(state: State) -> Result<State> {
    state
        .get_eos_action_receipts()
        .and_then(|action_receipts|
            generate_merkle_proof_from_action_receipts(
                &state.cli_args.arg_INDEX,
                &action_receipts,
            )
        )
        .and_then(|proof| state.add_merkle_proof(proof))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::{
        get_sample_eos_block,
        get_sample_action_receipts,
    };

    #[test]
    fn should_generate_merkle_proof_from_actions_receipts() {
        let expected_action_mroot = get_sample_eos_block()
            .unwrap()
            .action_mroot;
        let action_receipts = get_sample_action_receipts()
            .unwrap();
        let index = 3;
        let result = generate_merkle_proof_from_action_receipts(
            &index,
            &action_receipts,
        ).unwrap();
        let expected_result = vec![
            "1497fcf77837280811b604d3a8ec2a88eb5d81c2ef8de2570b68553993911929"
                .to_string(),
            "4108763bc6c4de8ddd5c70dde80140455921ad1abc325f6d7f6ea1f1a40bc76a"
                .to_string(),
            "1380b4d7305cc2ed39888d7c46ef4382b12d769f9816cb093ca4a7ca8cc9646a"
                .to_string(),
            "d0c9e46bcbf015158e8434a4d762f94654fc154988550bc6ef0915b6557cb32d"
                .to_string(),
            "899e5c8e163188e13492ece506899fa69ee4de72c3f14ec625cfaa6ec3304cd8"
                .to_string(),
        ];
        assert!(result == expected_result);
        let last = expected_result
            .last()
            .unwrap();
        assert!(last == &expected_action_mroot);
    }

}
