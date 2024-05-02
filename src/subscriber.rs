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

use async_std::task;
use async_trait::async_trait;
use common_uuri::ExampleType;
use std::{sync::Arc, time};
use up_client_zenoh::UPClientZenoh;
use up_rust::{Data, UListener, UMessage, UStatus, UTransport, UUri};
use zenoh::config::Config;

struct SubscriberListener;
#[async_trait]
impl UListener for SubscriberListener {
    async fn on_receive(&self, msg: UMessage) {
        if let Data::Value(v) = msg.payload.unwrap().data.unwrap() {
            let value = v.into_iter().map(|c| c as char).collect::<String>();
            let uri = msg.attributes.unwrap().source.unwrap().to_string();
            println!("Receiving {value} from {uri}");
        }
    }
    async fn on_error(&self, err: UStatus) {
        panic!("Internal Error: {err:?}");
    }
}

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    println!("uProtocol subscriber example");
    let subscriber = UPClientZenoh::new(
        Config::default(),
        common_uuri::authority(),
        common_uuri::entity(&ExampleType::Subscriber),
    )
    .await
    .unwrap();

    // create uuri
    let uuri = UUri {
        entity: Some(common_uuri::entity(&ExampleType::Publisher)).into(),
        resource: Some(common_uuri::pub_resource()).into(),
        ..Default::default()
    };

    println!("Register the listener...");
    subscriber
        .register_listener(uuri, Arc::new(SubscriberListener {}))
        .await
        .unwrap();

    loop {
        task::sleep(time::Duration::from_millis(1000)).await;
    }
}
