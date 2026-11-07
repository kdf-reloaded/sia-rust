//! The transaction-pool reply is read one transaction at a time: a transaction that cannot be read is left out and
//! does not fail the reply.
#[cfg(test)]
mod test {
    use crate::transport::endpoints::TxpoolTransactionsResponse;
    use crate::types::V2Transaction;
    use serde_json::{json, Value};

    /// A v2 transaction with a storage-proof resolution, as the pool of a node returns it.
    const STORAGE_PROOF_TX: &str = r#"{
                "siacoinInputs": [
                    {
                        "parent": {
                            "id": "ab5b0d4a5fc681b9b8bd276082cea42c3a3dc2359b8d2eb49722b544d2561115",
                            "stateElement": {
                                "leafIndex": 74391721
                            },
                            "siacoinOutput": {
                                "value": "17670215106248225520963228288",
                                "address": "15374e49010cac9840f86def0e8d63e3d0b1d11f951995f12c5152b908549f60c418613a8dd8"
                            },
                            "maturityHeight": 0
                        },
                        "satisfiedPolicy": {
                            "policy": {
                                "type": "uc",
                                "policy": {
                                    "timelock": 0,
                                    "publicKeys": [
                                        "ed25519:68271eda7adf5a80d154d15ccff3cebebcc447f0609cd580bb5c638aef227d43"
                                    ],
                                    "signaturesRequired": 1
                                }
                            },
                            "signatures": [
                                "e11e2edbc27b88461285283558e9b146ee620be1745ee9b60697f8e21409ea60ab1a4653710812f6836d3fefa08b96c1be16dcb87d3062034861f26915ecec06"
                            ]
                        }
                    }
                ],
                "siacoinOutputs": [
                    {
                        "value": "17670195106248225520963228288",
                        "address": "15374e49010cac9840f86def0e8d63e3d0b1d11f951995f12c5152b908549f60c418613a8dd8"
                    }
                ],
                "fileContractResolutions": [
                    {
                        "parent": {
                            "id": "c6eb6dca343a61a1a0f0b9f2a4939cdd51c297c987ca4b30b3333bdc0fc92081",
                            "stateElement": {
                                "leafIndex": 74348292
                            },
                            "v2FileContract": {
                                "capacity": 0,
                                "filesize": 0,
                                "fileMerkleRoot": "0000000000000000000000000000000000000000000000000000000000000000",
                                "proofHeight": 550434,
                                "expirationHeight": 550578,
                                "renterOutput": {
                                    "value": "5675257912250488405884928",
                                    "address": "b9d27c701886c5a86009a996eb518b3664973f2e812a4237f867f10483a178bd90fb4bd3244b"
                                },
                                "hostOutput": {
                                    "value": "1596749473887802825375744",
                                    "address": "15374e49010cac9840f86def0e8d63e3d0b1d11f951995f12c5152b908549f60c418613a8dd8"
                                },
                                "missedHostValue": "0",
                                "totalCollateral": "0",
                                "renterPublicKey": "ed25519:e8ccbabe231c1fc6e74ad974bec3ecf6ebca695d78715dae311bd7b6ec897471",
                                "hostPublicKey": "ed25519:6795c1b2ac73485a809d5c4218320426e3243cc700772cb0808c58e7723321d4",
                                "revisionNumber": 17,
                                "renterSignature": "7e1f07d5fb07f2b8ec3e137415165c4f55ad73b1302906b9fa7d22746a6a0a4483bd5d7717b9f8b022c41438c6b3046a7f2888f71a53703b5f9face20d8df507",
                                "hostSignature": "6db94f221b6ea9251c8546fb5f28d759cf0a5b2b389a2b5906aa1e1783316b74e4b41a05c6883f3d086551b4002a386c9b02f5e9048beb0437d6c4858f1ee00e"
                            }
                        },
                        "type": "storageProof",
                        "resolution": {
                            "proofIndex": {
                                "id": "00000000000000005fa0bf6b547eef003d7d667e573a579c5b6e3f842197b462",
                                "stateElement": {
                                    "leafIndex": 74393462
                                },
                                "chainIndex": {
                                    "height": 550434,
                                    "id": "00000000000000005fa0bf6b547eef003d7d667e573a579c5b6e3f842197b462"
                                }
                            },
                            "leaf": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                            "proof": []
                        }
                    }
                ],
                "minerFee": "20000000000000000000000"
            }"#;

    /// A plain v2 send: one input, two outputs.
    fn basic_send() -> Value {
        json!({
            "siacoinInputs": [{
                "parent": {
                    "id": "f59e395dc5cbe3217ee80eff60585ffc9802e7ca580d55297782d4a9b4e08589",
                    "stateElement": { "leafIndex": 3, "merkleProof": [
                        "ab0e1726444c50e2c0f7325eb65e5bd262a97aad2647d2816c39d97958d9588a"
                    ] },
                    "siacoinOutput": {
                        "value": "300000000000000000000000000000",
                        "address": "f7843ac265b037658b304468013da4fd0f304a1b73df0dc68c4273c867bfa38d01a7661a187f"
                    },
                    "maturityHeight": 145
                },
                "satisfiedPolicy": {
                    "policy": { "type": "uc", "policy": {
                        "timelock": 0,
                        "publicKeys": ["ed25519:cecc1507dc1ddd7295951c290888f095adb9044d1b73d696e6df065d683bd4fc"],
                        "signaturesRequired": 1
                    } },
                    "signatures": [
                        "f0a29ba576eb0dbc3438877ac1d3a6da4f3c4cbafd9030709c8a83c2fffa64f4dd080d37444261f023af3bd7a10a9597c33616267d5371bf2c0ade5e25e61903"
                    ]
                }
            }],
            "siacoinOutputs": [{
                "value": "1000000000000000000000000000",
                "address": "000000000000000000000000000000000000000000000000000000000000000089eb0d6a8a69"
            }],
            "minerFee": "0"
        })
    }

    /// An element the library cannot read: a list where a number belongs.
    fn unreadable() -> Value { json!({ "siacoinInputs": "not a list", "minerFee": [1, 2] }) }

    fn reply(v1: Value, v2: Value) -> Value {
        json!({
            "basis": { "height": 100, "id": "00000000000000005fa0bf6b547eef003d7d667e573a579c5b6e3f842197b462" },
            "transactions": v1,
            "v2transactions": v2
        })
    }

    fn read(reply: Value) -> TxpoolTransactionsResponse {
        serde_json::from_value::<TxpoolTransactionsResponse>(reply).expect("a pool reply is always readable")
    }

    #[test]
    fn happy_path_reads_every_transaction_in_order() {
        let storage_proof: Value = serde_json::from_str(STORAGE_PROOF_TX).unwrap();
        let response = read(reply(Value::Null, json!([basic_send(), storage_proof, basic_send()])));
        assert!(response.transactions.is_empty());
        assert_eq!(response.v2transactions.len(), 3);
        let expected_first: V2Transaction = serde_json::from_value(basic_send()).unwrap();
        assert_eq!(response.v2transactions[0], expected_first);
        assert_eq!(response.v2transactions[2], expected_first);
    }

    #[test]
    fn one_unreadable_v2_transaction_is_left_out() {
        let response = read(reply(Value::Null, json!([basic_send(), unreadable(), basic_send()])));
        assert_eq!(response.v2transactions.len(), 2);
    }

    #[test]
    fn an_unreadable_first_and_last_element_are_left_out() {
        let response = read(reply(Value::Null, json!([unreadable(), basic_send(), unreadable()])));
        assert_eq!(response.v2transactions.len(), 1);
    }

    #[test]
    fn a_list_of_only_unreadable_transactions_is_an_empty_list() {
        let response = read(reply(json!([unreadable()]), json!([unreadable(), unreadable()])));
        assert!(response.transactions.is_empty());
        assert!(response.v2transactions.is_empty());
    }

    #[test]
    fn an_unreadable_v1_transaction_is_left_out_and_the_v2_list_is_kept() {
        let response = read(reply(json!([{ "siacoinInputs": 5 }]), json!([basic_send()])));
        assert!(response.transactions.is_empty());
        assert_eq!(response.v2transactions.len(), 1);
    }

    #[test]
    fn a_null_storage_proof_list_is_read_as_an_empty_proof() {
        let mut storage_proof: Value = serde_json::from_str(STORAGE_PROOF_TX).unwrap();
        storage_proof["fileContractResolutions"][0]["resolution"]["proof"] = Value::Null;
        let response = read(reply(Value::Null, json!([basic_send(), storage_proof])));
        assert_eq!(response.v2transactions.len(), 2);
    }

    #[test]
    fn missing_and_null_lists_are_empty_lists() {
        let response = read(json!({ "basis": null }));
        assert!(response.transactions.is_empty());
        assert!(response.v2transactions.is_empty());
        let response = read(reply(Value::Null, Value::Null));
        assert!(response.transactions.is_empty());
        assert!(response.v2transactions.is_empty());
    }

    #[test]
    fn a_reply_that_is_not_a_pool_reply_still_fails() {
        // Only an element of a list is tolerated; a list that is not a list is still an error.
        let result = serde_json::from_value::<TxpoolTransactionsResponse>(json!({ "v2transactions": "no list" }));
        assert!(result.is_err());
    }
}
