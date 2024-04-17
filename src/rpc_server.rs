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
use async_std::task::{self, block_on};
use async_trait::async_trait;
use chrono::Utc;
use std::{sync::Arc, time};
use up_client_zenoh::UPClientZenoh;
use up_rust::{
    Data, Number, UAuthority, UEntity, UListener, UMessage, UMessageBuilder, UPayloadFormat,
    UResourceBuilder, UStatus, UTransport, UUIDBuilder, UUri,
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
            UAuthority {
                name: Some("auth_name".to_string()),
                number: Some(Number::Id(vec![1, 2, 3, 4])),
                ..Default::default()
            },
            UEntity {
                name: "entity_rpc_server".to_string(),
                id: Some(3),
                version_major: Some(1),
                version_minor: None,
                ..Default::default()
            },
        )
        .await
        .unwrap(),
    );

    // create uuri
    let uuri = UUri {
        entity: Some(UEntity {
            name: "test_rpc.app".to_string(),
            version_major: Some(1),
            id: Some(1234),
            ..Default::default()
        })
        .into(),
        resource: Some(UResourceBuilder::for_rpc_request(
            Some("getTime".to_string()),
            Some(5678),
        ))
        .into(),
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
