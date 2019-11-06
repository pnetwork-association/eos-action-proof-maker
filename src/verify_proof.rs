use crate::{
    state::State,
    error::AppError,
    eos_merkle_utils::verify_merkle_proof,
    types::{
        Result,
        MerkleProof,
    },
};

fn verify_proof(merkle_proof: &MerkleProof) -> Result<()> {
    match verify_merkle_proof(&merkle_proof) {
        Ok(true) => Ok(()),
        _ => Err(AppError::Custom(
            "✘ Error verifying generated merkle proof!".to_string()
        )),
    }
}

pub fn verify_proof_in_state(state: State) -> Result<State> {
    state
        .get_merkle_proof()
        .and_then(verify_proof)
        .and_then(|_| Ok(state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::get_sample_merkle_proof;

    #[test]
    fn should_verify_valid_merkle_proof() {
        let proof = get_sample_merkle_proof()
            .unwrap();
        if let Err(e) = verify_proof(&proof) {
            panic!("Should not error verifying valid proof {}", e);
        }
    }

    #[test]
    fn should_fail_to_verify_invalid_merkle_proof() {
        let mut proof = get_sample_merkle_proof()
            .unwrap();
        proof.remove(1);
        if let Ok(_) = verify_proof(&proof) {
            panic!("Should error verifying invalid proof!");
        }
    }
}
