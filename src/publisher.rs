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
use common_uuri::ExampleType;
use std::time;
use up_client_zenoh::UPClientZenoh;
use up_rust::{UMessageBuilder, UPayloadFormat, UTransport, UUIDBuilder, UUri};

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    println!("uProtocol publisher example");
    let publisher = UPClientZenoh::new(
        common_uuri::get_zenoh_config(),
        common_uuri::authority(),
        common_uuri::entity(&ExampleType::Publisher),
    )
    .await
    .unwrap();

    // create uuri
    let uuri = UUri {
        entity: Some(common_uuri::entity(&ExampleType::Publisher)).into(),
        resource: Some(common_uuri::pub_resource()).into(),
        ..Default::default()
    };

    let mut cnt: u64 = 0;
    loop {
        let data = format!("{cnt}");
        let umessage = UMessageBuilder::publish(uuri.clone())
            .with_message_id(UUIDBuilder::build())
            .build_with_payload(
                data.as_bytes().to_vec().into(),
                UPayloadFormat::UPAYLOAD_FORMAT_TEXT,
            )
            .unwrap();
        println!("Sending {data} to {uuri}...");
        publisher.send(umessage).await.unwrap();
        task::sleep(time::Duration::from_millis(1000)).await;
        cnt += 1;
    }
}
