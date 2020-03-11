use crate::{
    state::State,
    eos_merkle_utils::generate_merkle_proof,
    types::{
        Result,
        MerkleProof,
        EosActionReceipts,
    },
};

pub fn generate_merkle_proof_from_action_receipts(
    index: &usize,
    action_receipts: &EosActionReceipts,
) -> Result<MerkleProof> {
    generate_merkle_proof(
        index.clone(),
        action_receipts
            .iter()
            .map(|action_receipt| action_receipt.to_digest())
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
    use crate::test_utils::{
        MERKLE_PROOF_INDEX,
        get_sample_eos_block_n,
        get_sample_merkle_proof_n,
        get_sample_action_receipts_n,
    };

    #[test]
    fn should_generate_merkle_proof_from_actions_receipts() {
        let expected_action_mroot = get_sample_eos_block_n(1)
            .unwrap()
            .action_mroot;
        let action_receipts = get_sample_action_receipts_n(1)
            .unwrap();
        let result = generate_merkle_proof_from_action_receipts(
            &MERKLE_PROOF_INDEX,
            &action_receipts,
        ).unwrap();
        let expected_result = get_sample_merkle_proof_n(1)
            .unwrap();
        assert!(result == expected_result);
        let last = expected_result
            .last()
            .unwrap();
        assert!(last == &expected_action_mroot);
    }
}
