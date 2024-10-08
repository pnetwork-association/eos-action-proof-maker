use crate::error::AppError;
use crate::types::{Byte, Bytes, MerkleProof, Result};
use bitcoin_hashes::{sha256, Hash};

pub type CanonicalLeft = Bytes;
pub type CanonicalRight = Bytes;
pub type Sha256Hash = bitcoin_hashes::sha256::Hash;
pub type CanonicalPair = (CanonicalLeft, CanonicalRight);

fn set_first_bit_of_byte_to_zero(mut byte: Byte) -> Byte {
    // Left
    byte &= 0b0111_1111;
    byte
}

fn set_first_bit_of_byte_to_one(mut byte: Byte) -> Byte {
    // Right
    byte |= 0b1000_0000;
    byte
}

fn set_first_bit_of_hash_to_one(mut hash: Bytes) -> Bytes {
    hash[0] = set_first_bit_of_byte_to_one(hash[0]);
    hash
}

fn set_first_bit_of_hash_to_zero(mut hash: Bytes) -> Bytes {
    hash[0] = set_first_bit_of_byte_to_zero(hash[0]);
    hash
}

fn make_canonical_left(hash: Bytes) -> CanonicalLeft {
    set_first_bit_of_hash_to_zero(hash)
}

fn make_canonical_right(hash: Bytes) -> CanonicalRight {
    set_first_bit_of_hash_to_one(hash)
}

fn is_canonical_left(hash: &Bytes) -> bool {
    hash[0] & 0b1000_0000 == 0
}

fn is_canonical_right(hash: &Bytes) -> bool {
    !is_canonical_left(hash)
}

fn make_canonical_pair(l: Bytes, r: Bytes) -> CanonicalPair {
    (make_canonical_left(l), make_canonical_right(r))
}

fn concatenate_canonical_pair(mut pair: CanonicalPair) -> Bytes {
    pair.0.append(&mut pair.1);
    pair.0
}

fn hash_canonical_pair(pair: CanonicalPair) -> Sha256Hash {
    sha256::Hash::hash(&concatenate_canonical_pair(pair))
}

fn make_and_hash_canonical_pair(l: Bytes, r: Bytes) -> Bytes {
    hash_canonical_pair(make_canonical_pair(l, r)).to_vec()
}

pub fn get_merkle_digest(mut leaves: Vec<Bytes>) -> Bytes {
    if leaves.is_empty() {
        return vec![0x00]; // TODO Need a type for this!
    }
    while leaves.len() > 1 {
        if leaves.len() % 2 != 0 {
            let last = leaves[leaves.len() - 1].clone();
            leaves.push(last);
        }
        for i in 0..(leaves.len() / 2) {
            leaves[i] = hash_canonical_pair(make_canonical_pair(
                leaves[2 * i].clone(),
                leaves[(2 * i) + 1].clone(),
            ))
            .to_vec();
        }
        leaves.resize(leaves.len() / 2, vec![0x00]);
    }
    leaves[0].clone()
}

pub fn generate_merkle_proof(mut index: usize, mut leaves: Vec<Bytes>) -> Result<MerkleProof> {
    let mut proof = Vec::new();
    proof.push(hex::encode(leaves[index].clone()));
    match index < leaves.len() {
        false => Err(AppError::Custom(format!(
            "✘ Error generating merkle proof!\n{}",
            "✘ Index out of bounds!"
        ))),
        true => {
            while leaves.len() > 1 {
                if leaves.len() % 2 != 0 {
                    let last = leaves[leaves.len() - 1].clone();
                    leaves.push(last)
                }
                for i in 0..leaves.len() / 2 {
                    if index / 2 == i {
                        if index % 2 != 0 {
                            proof.push(hex::encode(make_canonical_left(leaves[2 * i].clone())))
                        } else {
                            proof.push(hex::encode(make_canonical_right(leaves[2 * i + 1].clone())))
                        }
                        index /= 2;
                    }
                    leaves[i] = hash_canonical_pair(make_canonical_pair(
                        leaves[2 * i].clone(),
                        leaves[2 * i + 1].clone(),
                    ))
                    .to_vec()
                }
                leaves.resize(leaves.len() / 2, vec![0x00]);
            }
            proof.push(hex::encode(leaves[0].clone()));
            Ok(proof)
        }
    }
}

pub fn verify_merkle_proof(merkle_proof: &MerkleProof) -> Result<bool> {
    let mut leaves = Vec::new();
    for proof in merkle_proof.iter() {
        leaves.push(hex::decode(proof.clone())?)
    }
    let mut node = leaves[0].clone();
    for leaf in leaves.iter().take(leaves.len() - 1).skip(1) {
        if is_canonical_right(leaf) {
            node = make_and_hash_canonical_pair(node, leaf.clone());
        } else {
            node = make_and_hash_canonical_pair(leaf.clone(), node);
        }
    }
    Ok(Some(&node) == leaves.last())
}

/* FIXME Reinstate!
#[cfg(test)]
mod tests {
    use super::*;
    use eos_chain::{
        AccountName, Action, ActionName, ActionReceipt, utils::flat_map::FlatMap, PermissionLevel,
        PermissionName, SerializeData,
    };
    use hex;
    use std::str::FromStr;

    fn get_expected_digest_1() -> &'static str {
        "9b9babebfbdff48ce4002b5f3c7f999c0ee74707b6d121c47ef5db68c6be7262"
    }

    fn get_expected_digest_2() -> &'static str {
        "122cd09d66ca7df007a35bd9c9be5484833f1a69ad0c8527c3e2a56b6955e761"
    }

    fn get_expected_digest_bytes_1() -> Bytes {
        hex::decode(get_expected_digest_1()).unwrap()
    }

    fn get_expected_digest_bytes_2() -> Bytes {
        hex::decode(get_expected_digest_2()).unwrap()
    }

    fn get_expected_first_byte_1() -> Byte {
        0b0001_1011
    }

    fn get_expected_first_byte_2() -> Byte {
        0b1001_0010
    }

    fn get_sample_canonical_pair() -> CanonicalPair {
        make_canonical_pair(get_expected_digest_bytes_1(), get_expected_digest_bytes_2())
    }

    fn get_sample_action_receipts() -> Vec<ActionReceipt> {
        vec![
            ActionReceipt::new(
                "eosio",
                "3b434aa9331f5e2a0e7a0060d576fa6688406667100bdf3458104dede44ec4e9",
                62826453,
                12,
                503081363,
                10,
                vec![FlatMap::new(AccountName::from_str("eosio").unwrap(), 61285932)],
            )
            .unwrap(),
            ActionReceipt::new(
                "pokerpokerts",
                "3d380413463e8716ef9c1f8c853dfab0c70f209cce75cae9a5b74e4e678a68a0",
                241512,
                4,
                503081364,
                30,
                vec![FlatMap::new(AccountName::from_str("pokerpokerts").unwrap(), 241552)],
            )
            .unwrap(),
            ActionReceipt::new(
                "oracleoracle",
                "065527f0429dfa9bb79575ec5270b20f714fb9e61a9ce6ba9c86b2e69a773f82",
                531231,
                2,
                503081365,
                2,
                vec![FlatMap::new(AccountName::from_str("feeder111112").unwrap(), 152730)],
            )
            .unwrap(),
            ActionReceipt::new(
                "dvmh1tbb1him",
                "18e42aa86473509cf620764ca606136b037e1a8ee6fb8efaa8fa657c7fa2fffc",
                805647,
                2,
                503081366,
                1,
                vec![FlatMap::new(AccountName::from_str("dvmh1tbb1him").unwrap(), 805667)],
            )
            .unwrap(),
        ]
    }

    fn get_sample_action_digests() -> Vec<Bytes> {
        get_sample_action_receipts()
            .into_iter()
            .map(|receipt| receipt.to_digest())
            .collect()
    }

    #[test]
    fn should_set_first_bit_of_byte_to_zero() {
        let byte = 0b1011_1011;
        let expected_result = 0b0011_1011;
        let result = set_first_bit_of_byte_to_zero(byte);
        assert!(result == expected_result);
    }

    #[test]
    fn should_set_first_bit_of_byte_to_one() {
        let byte = 0b0011_0011;
        let expected_result = 0b1011_0011;
        let result = set_first_bit_of_byte_to_one(byte);
        assert!(result == expected_result);
    }

    #[test]
    fn should_set_first_bit_of_hash_to_one() {
        let hash = get_expected_digest_bytes_2();
        let result = set_first_bit_of_hash_to_one(hash.clone());
        for i in 0..hash.len() {
            if i == 0 {
                assert!(result[i] == get_expected_first_byte_2());
            } else {
                assert!(result[i] == hash[i]);
            }
        }
    }

    #[test]
    fn should_set_first_bit_of_hash_to_zero() {
        let hash = get_expected_digest_bytes_1();
        let result = set_first_bit_of_hash_to_zero(hash.clone());
        for i in 0..hash.len() {
            if i == 0 {
                assert!(result[i] == get_expected_first_byte_1());
            } else {
                assert!(result[i] == hash[i]);
            }
        }
    }

    #[test]
    fn should_make_hash_canonical_right() {
        let hash = get_expected_digest_bytes_2();
        let result = make_canonical_right(hash.clone());
        for i in 0..hash.len() {
            if i == 0 {
                assert!(result[i] == get_expected_first_byte_2());
            } else {
                assert!(result[i] == hash[i]);
            }
        }
    }

    #[test]
    fn should_make_hash_canonical_left() {
        let hash = get_expected_digest_bytes_1();
        let result = make_canonical_left(hash.clone());
        for i in 0..hash.len() {
            if i == 0 {
                assert!(result[i] == get_expected_first_byte_1());
            } else {
                assert!(result[i] == hash[i]);
            }
        }
    }

    #[test]
    fn canonical_left_hash_should_be_canonical_left() {
        let hash = get_expected_digest_bytes_1();
        let canonical_left_hash = make_canonical_left(hash.clone());
        let is_left = is_canonical_left(&canonical_left_hash);
        let is_right = is_canonical_right(&canonical_left_hash);
        assert!(is_left);
        assert!(!is_right);
    }

    #[test]
    fn canonical_right_hash_should_be_canonical_right() {
        let hash = get_expected_digest_bytes_2();
        let canonical_right_hash = make_canonical_right(hash.clone());
        let is_left = is_canonical_left(&canonical_right_hash);
        let is_right = is_canonical_right(&canonical_right_hash);
        assert!(!is_left);
        assert!(is_right);
    }

    #[test]
    fn should_get_correct_action_digest() {
        let account_name = AccountName::from_str("provabletokn").unwrap();
        let action_name = ActionName::from_str("event").unwrap();
        let actor = AccountName::from_str("provabletokn").unwrap();
        let permission = PermissionName::from_str("active").unwrap();
        let permission_level = PermissionLevel { actor, permission };
        let authorization = vec![permission_level];
        let data = hex::decode(
            "e0d2b86b1a3962343021cd2a1eb3e9ad672b00000000000004454f53000000002a3078303236644336413433353631444138413641373735353338623139324133653933366330463239422301000000000000"
            ).unwrap();
        let action = Action::new(account_name, action_name, authorization, data);
        let serialized_action = &action.to_serialize_data();
        let result = sha256::Hash::hash(serialized_action).to_string();
        assert!(result == get_expected_digest_1().to_string());
    }

    #[test]
    fn should_make_canonical_pair() {
        let digest_1 = get_expected_digest_bytes_1();
        let digest_2 = get_expected_digest_bytes_2();
        let result = make_canonical_pair(digest_1.clone(), digest_2.clone());
        for i in 0..result.0.len() {
            if i == 0 {
                assert!(result.0[i] == get_expected_first_byte_1());
            } else {
                assert!(result.0[i] == digest_1[i]);
            }
        }
        for i in 0..result.1.len() {
            if i == 0 {
                assert!(result.1[i] == get_expected_first_byte_2());
            } else {
                assert!(result.1[i] == digest_2[i]);
            }
        }
    }

    #[test]
    fn should_hash_canonical_pair() {
        let expected_result = "a26284468e89fe4a5cce763ca3b3d3d37d5fcb35f289c63f0558487ec57ace28";
        let canonical_pair = get_sample_canonical_pair();
        let result = hash_canonical_pair(canonical_pair);
        assert!(result.to_string() == expected_result);
    }

    #[test]
    fn should_serialize_a_simple_action_receipt_correctly() {
        let expected_result =
            "6cd473b189a292bd520cac3430cc7934273da81cc3417376194a5d757b4abdc8".to_string();
        let result = ActionReceipt::new(
            "eosio",
            "a6a370c6569034a4cc41935dd88f83d1c64e0414580872f29d87f69fe7a5d769",
            60725518,
            12,
            498637597,
            10,
            vec![FlatMap::new(AccountName::from_str("eosio").unwrap(), 59191700)],
        )
        .unwrap()
        .to_digest();
        assert!(hex::encode(result) == expected_result);
    }

    #[test]
    fn should_get_merkle_root_for_an_even_number_of_action_receipts() {
        // NOTE: Test vector = https://jungle.bloks.io/block/58316764
        let expected_result = "2f013d3ed57c89f1824772d18a4a74c043574bad47e9c6f088136e7595511810";
        let action_digest_1 = ActionReceipt::new(
            "eosio",
            "8e3e721a497dbae5e5fde0bb43e9086628809efaf102b763a3e9820adce9ce8f",
            62815613,
            12,
            503056735,
            10,
            vec![FlatMap::new(AccountName::from_str("eosio").unwrap(), 61275209)],
        )
        .unwrap()
        .to_digest();
        let action_digest_2 = ActionReceipt::new(
            "provabletokn",
            "4b991cebb3e6667b242aca3fb011623cd8ce2be2e8c24958da551c7b3ba68903",
            2884,
            48,
            503056736,
            80,
            vec![FlatMap::new(AccountName::from_str("provabletokn").unwrap(), 3090)],
        )
        .unwrap()
        .to_digest();
        let result = get_merkle_digest(vec![action_digest_1, action_digest_2]);
        assert!(hex::encode(result) == expected_result);
    }

    #[test]
    fn should_get_merkle_root_for_an_odd_number_of_action_receipts_gt_one() {
        // NOTE: Test vector = https://jungle.bloks.io/block/58319528
        let expected_result = "593f54cbc0b877b30cec5e510838b2b16ca00aca43e21d204d21eb8e8f947aa0";
        let action_digest_1 = ActionReceipt::new(
            "eosio",
            "23ab74b930cceea6061e1c4580ec988bf483a77e225cfca254d832928b4d1b36",
            62818486,
            12,
            503062766,
            10,
            vec![FlatMap::new(AccountName::from_str("eosio").unwrap(), 61277973)],
        )
        .unwrap()
        .to_digest();
        let action_digest_2 = ActionReceipt::new(
            "eosebetbullj",
            "b9243d8513e25705e89d7ccd0491f4a57d07b9866fd89d3446887af852cfed15",
            1440226,
            215,
            503062767,
            215,
            vec![FlatMap::new(AccountName::from_str("eosebetbullj").unwrap(), 1440687)],
        )
        .unwrap()
        .to_digest();
        let action_digest_3 = ActionReceipt::new(
            "dvmh1tbb1him",
            "4bd1d3e987cd13e3d108a9a0cd185bf022cb1a826f69f163fcd109db54ba799f",
            804629,
            2,
            503062768,
            1,
            vec![FlatMap::new(AccountName::from_str("dvmh1tbb1him").unwrap(), 804649)],
        )
        .unwrap()
        .to_digest();
        let result = get_merkle_digest(vec![action_digest_1, action_digest_2, action_digest_3]);
        assert!(hex::encode(result) == expected_result);
    }

    #[test]
    fn should_get_action_mroot_when_action_has_gt_one_auth_sequence() {
        // NOTE: Test vector = https://jungle.bloks.io/block/58345436
        let expected_result = "f93a91688d12170c24807d4bd507cf52dcde962ae4a41a86fe55231dee4df348";
        let action_receipt_1 = ActionReceipt::new(
            "eosio",
            "2d5371b958af052629f3fb62ede1bfcd94703675bc734535bf87fb615284dba3",
            62844592,
            12,
            503124645,
            10,
            vec![FlatMap::new(AccountName::from_str("eosio").unwrap(), 61303891)],
        )
        .unwrap()
        .to_digest();
        let action_receipt_2 = ActionReceipt::new(
            "authsequence",
            "ae341469a7b3936c70e9684a42ef8fc1975f1bb2fe1f3b0b1105eda7d3a6276a",
            10,
            1,
            503124646,
            1,
            vec![
                FlatMap::new(AccountName::from_str("othrsequence").unwrap(), 14),
                FlatMap::new(AccountName::from_str("rick11111111").unwrap(), 268),
            ],
        )
        .unwrap()
        .to_digest();
        let result = get_merkle_digest(vec![action_receipt_1, action_receipt_2]);
        assert!(hex::encode(result) == expected_result);
    }

    #[test]
    fn should_get_action_mroot_for_four_actions_correctly() {
        let digests = get_sample_action_digests();
        let expected_result = "8b4e5e5d3e7587065896d0076d65c72e03c11a9159d414eb3a2363b59108116a";
        let result = get_merkle_digest(digests);
        assert!(hex::encode(result) == expected_result);
    }

    #[test]
    fn should_generate_merkle_proof_correctly() {
        let index = 2;
        let digests = get_sample_action_digests();
        let expected_result = vec![
            "41a91de4e161bc10ff6f7173822cf3f6417dd9bfc3dc88cf8bdd8196322837ce",
            "d14a4b2157aaaa4fec077069a8617c53b4e3f75c142a6aea5aa24bd031578e96",
            "3d8ddd7684e1f38a0f3c6880d179fc43ae730f9363eafe842cc85484450e2613",
            "8b4e5e5d3e7587065896d0076d65c72e03c11a9159d414eb3a2363b59108116a",
        ];
        let result = generate_merkle_proof(index, digests).unwrap();
        for i in 0..expected_result.len() {
            assert!(expected_result[i] == result[i]);
        }
    }

    #[test]
    fn should_verify_merkle_proof_correctly() {
        let index = 2;
        let digests = get_sample_action_digests();
        let proof = generate_merkle_proof(index, digests).unwrap();
        let result = verify_merkle_proof(&proof).unwrap();
        assert!(result);
    }
}
*/
