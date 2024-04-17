//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
pub mod common_uuri;

use common_uuri::ExampleType;
use up_client_zenoh::UPClientZenoh;
use up_rust::{CallOptions, Data, RpcClient, UPayload, UPayloadFormat, UUri};
use zenoh::config::Config;

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    println!("uProtocol RPC client example");
    let rpc_client = UPClientZenoh::new(
        Config::default(),
        common_uuri::authority(),
        common_uuri::entity(&ExampleType::RpcClient),
    )
    .await
    .unwrap();

    // create uuri
    let uuri = UUri {
        entity: Some(common_uuri::entity(&ExampleType::RpcServer)).into(),
        resource: Some(common_uuri::rpc_resource()).into(),
        ..Default::default()
    };

    // create uPayload
    let data = String::from("GetCurrentTime");
    let payload = UPayload {
        length: Some(0),
        format: UPayloadFormat::UPAYLOAD_FORMAT_TEXT.into(),
        data: Some(Data::Value(data.as_bytes().to_vec())),
        ..Default::default()
    };

    // invoke RPC method
    println!("Send request to {uuri}");
    let result = rpc_client
        .invoke_method(
            uuri,
            payload,
            CallOptions {
                ttl: 1000,
                ..Default::default()
            },
        )
        .await;

    // process the result
    if let Data::Value(v) = result.unwrap().payload.unwrap().data.unwrap() {
        let value = v.into_iter().map(|c| c as char).collect::<String>();
        println!("Receive {value}");
    } else {
        println!("Failed to get result from invoke_method.");
    }
}
