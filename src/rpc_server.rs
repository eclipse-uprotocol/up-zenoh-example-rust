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

use async_std::task::{self, block_on};
use async_trait::async_trait;
use chrono::Utc;
use common_uuri::ExampleType;
use std::{sync::Arc, time};
use up_client_zenoh::UPClientZenoh;
use up_rust::{
    Data, UListener, UMessage, UMessageBuilder, UPayloadFormat, UStatus, UTransport, UUIDBuilder,
    UUri,
};
use zenoh::config::Config;

struct RpcListener {
    up_client: Arc<UPClientZenoh>,
}
impl RpcListener {
    fn new(up_client: Arc<UPClientZenoh>) -> Self {
        RpcListener { up_client }
    }
}
#[async_trait]
impl UListener for RpcListener {
    async fn on_receive(&self, msg: UMessage) {
        let UMessage {
            attributes,
            payload,
            ..
        } = msg;
        // Build the payload to send back
        if let Data::Value(v) = payload.unwrap().data.unwrap() {
            let value = v.into_iter().map(|c| c as char).collect::<String>();
            let source = attributes.clone().unwrap().source.unwrap();
            let sink = attributes.clone().unwrap().sink.unwrap();
            println!("Receive {value} from {source} to {sink}");
        }
        // Send back result
        let umessage = UMessageBuilder::response_for_request(&attributes)
            .with_message_id(UUIDBuilder::build())
            .build_with_payload(
                // Get current time
                format!("{}", Utc::now()).as_bytes().to_vec().into(),
                UPayloadFormat::UPAYLOAD_FORMAT_TEXT,
            )
            .unwrap();
        block_on(self.up_client.send(umessage)).unwrap();
    }
    async fn on_error(&self, err: UStatus) {
        panic!("Internal Error: {err:?}");
    }
}

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    println!("uProtocol RPC server example");
    let rpc_server = Arc::new(
        UPClientZenoh::new(
            Config::default(),
            common_uuri::authority(),
            common_uuri::entity(&ExampleType::RpcServer),
        )
        .await
        .unwrap(),
    );

    // create uuri
    let uuri = UUri {
        entity: Some(common_uuri::entity(&ExampleType::RpcServer)).into(),
        resource: Some(common_uuri::rpc_resource()).into(),
        ..Default::default()
    };

    println!("Register the listener...");
    rpc_server
        .register_listener(uuri, Arc::new(RpcListener::new(rpc_server.clone())))
        .await
        .unwrap();

    loop {
        task::sleep(time::Duration::from_millis(1000)).await;
    }
}
