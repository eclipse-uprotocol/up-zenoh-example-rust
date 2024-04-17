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

use up_rust::{Number, UAuthority, UEntity, UResource, UResourceBuilder};

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
