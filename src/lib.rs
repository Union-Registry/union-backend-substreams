mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const UNIONBACKEND_TRACKED_CONTRACT: [u8; 20] = hex!("031f0da3d62302fedb5f23a539fb0ded62b64352");

fn map_unionbackend_events(blk: &eth::Block, events: &mut contract::Events) {
    events.unionbackend_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == UNIONBACKEND_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::unionbackend_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::UnionbackendInitialized {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.unionbackend_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == UNIONBACKEND_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::unionbackend_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::UnionbackendOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.unionbackend_union_accepteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == UNIONBACKEND_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::unionbackend_contract::events::UnionAccepted::match_and_decode(log) {
                        return Some(contract::UnionbackendUnionAccepted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            attestation_uid: event.attestation_uid.to_u64(),
                            union_id: event.union_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.unionbackend_union_proposeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == UNIONBACKEND_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::unionbackend_contract::events::UnionProposed::match_and_decode(log) {
                        return Some(contract::UnionbackendUnionProposed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            union_id: event.union_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.unionbackend_union_revokeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == UNIONBACKEND_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::unionbackend_contract::events::UnionRevoked::match_and_decode(log) {
                        return Some(contract::UnionbackendUnionRevoked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            union_id: event.union_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_unionbackend_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.unionbackend_call_accept_unions.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::AcceptUnion::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::AcceptUnion::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendAcceptUnionCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                secret: decoded_call.secret,
                                token_id: decoded_call.token_id.to_string(),
                                union_id: decoded_call.union_id.to_string(),
                                vow: decoded_call.vow,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::Initialize::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_rings_contract: decoded_call.u_rings_contract,
                                u_schema: decoded_call.u_schema.to_u64(),
                                u_sp_instance_address: decoded_call.u_sp_instance_address,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_on_erc1155_batch_receiveds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::OnErc1155BatchReceived::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::OnErc1155BatchReceived::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::unionbackend_contract::functions::OnErc1155BatchReceived::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UnionbackendOnErc1155BatchReceivedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: Vec::from(output_param0),
                                param0: decoded_call.param0,
                                param1: decoded_call.param1,
                                param2: decoded_call.param2.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                param3: decoded_call.param3.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                param4: decoded_call.param4,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_on_erc1155_receiveds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::OnErc1155Received::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::OnErc1155Received::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::unionbackend_contract::functions::OnErc1155Received::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UnionbackendOnErc1155ReceivedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: Vec::from(output_param0),
                                param0: decoded_call.param0,
                                param1: decoded_call.param1,
                                param2: decoded_call.param2.to_string(),
                                param3: decoded_call.param3.to_string(),
                                param4: decoded_call.param4,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_propose_unions.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::ProposeUnion::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::ProposeUnion::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendProposeUnionCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                secret_hash: Vec::from(decoded_call.secret_hash),
                                token_id: decoded_call.token_id.to_string(),
                                vow: decoded_call.vow,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_renounce_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::RenounceOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::RenounceOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_revoke_unions.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::RevokeUnion::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::RevokeUnion::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendRevokeUnionCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                reason: decoded_call.reason,
                                union_id: decoded_call.union_id.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_set_rings_contracts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::SetRingsContract::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::SetRingsContract::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendSetRingsContractCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_rings_contract: decoded_call.u_rings_contract,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_set_schemas.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::SetSchema::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::SetSchema::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendSetSchemaCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_schema: decoded_call.u_schema.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_set_sp_contracts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::SetSpContract::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::SetSpContract::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendSetSpContractCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_sp_instance_address: decoded_call.u_sp_instance_address,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.unionbackend_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == UNIONBACKEND_TRACKED_CONTRACT && abi::unionbackend_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::unionbackend_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UnionbackendTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_unionbackend_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
let mut calls = contract::Calls::default();
    map_unionbackend_calls(&blk, &mut calls);
    Ok(calls)
}

