/********************************************************************************
 * Copyright (c) 2024 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/
use up_rust::{Number, UAuthority, UEntity, UResource, UResourceBuilder};
use zenoh::config::Config;

pub enum ExampleType {
    Publisher,
    Subscriber,
    RpcServer,
    RpcClient,
}

#[allow(clippy::must_use_candidate)]
pub fn authority() -> UAuthority {
    UAuthority {
        name: Some("auth_name".to_string()),
        number: Some(Number::Id(vec![1, 2, 3, 4])),
        ..Default::default()
    }
}

#[allow(clippy::must_use_candidate)]
pub fn entity(example_type: &ExampleType) -> UEntity {
    let (name, id) = match example_type {
        ExampleType::Publisher => ("publisher", 1),
        ExampleType::Subscriber => ("subscriber", 2),
        ExampleType::RpcServer => ("rpc_server", 3),
        ExampleType::RpcClient => ("rpc_client", 4),
    };
    UEntity {
        name: name.to_string(),
        id: Some(1),
        version_major: Some(id),
        ..Default::default()
    }
}

#[allow(clippy::must_use_candidate)]
pub fn pub_resource() -> UResource {
    UResource {
        name: "door".to_string(),
        instance: Some("front_left".to_string()),
        message: Some("Door".to_string()),
        id: Some(5678),
        ..Default::default()
    }
}

#[allow(clippy::must_use_candidate)]
pub fn rpc_resource() -> UResource {
    UResourceBuilder::for_rpc_request(Some("getTime".to_string()), Some(5678))
}

#[allow(clippy::must_use_candidate, clippy::missing_panics_doc)]
pub fn get_zenoh_config() -> Config {
    // Load the config from file path
    // Config Examples: https://github.com/eclipse-zenoh/zenoh/blob/0.10.1-rc/DEFAULT_CONFIG.json5
    // let mut zenoh_cfg = Config::from_file("./DEFAULT_CONFIG.json5").unwrap();

    // Loat the default config struct
    let mut zenoh_cfg = Config::default();
    // You can choose from Router, Peer, Client
    zenoh_cfg
        .set_mode(Some(zenoh::config::WhatAmI::Peer))
        .unwrap();

    zenoh_cfg
}
