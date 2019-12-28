/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */

use anyhow;
use rpc::types as rpctypes;

use crate::types;

impl From<rpctypes::Coordinate> for types::Coordinate {
    fn from(other: rpctypes::Coordinate) -> Self {
        Self {
            x: other.x as f64,
            y: other.y as f64,
            z: other.z as f64,
        }
    }
}

impl From<rpctypes::System> for types::System {
    fn from(other: rpctypes::System) -> Self {
        Self {
            id: other.id.into(),
            name: other.name,
            coordinate: other.coordinate.into(),
            security: other.security.into(),
        }
    }
}

fn conv_undirected_to_directed(other: rpctypes::UndirectedConnection) -> Vec<types::Connection> {
    let id1: types::SystemId = other.0.into();
    let id2: types::SystemId = other.1.into();
    // TODO: Fixme
    let stargate_type = types::ConnectionType::Stargate(types::StargateType::Local);
    let a = types::Connection { from: id1.clone(), to: id2.clone(), type_: stargate_type.clone() };
    let b = types::Connection { from: id2.clone(), to: id1.clone(), type_: stargate_type.clone() };
    vec![a, b]
}

pub struct RpcTypeBuilder {
    systems: rpctypes::Systems,
    connections: rpctypes::Connections,
}

impl RpcTypeBuilder {
    pub fn new(systems: rpctypes::Systems, connections: rpctypes::Connections) -> Self {
        Self {
            systems,
            connections,
        }
    }

    pub fn build(self) -> anyhow::Result<types::Universe> {
        let systems = self.systems
            .into_iter()
            .map(|s| s.into())
            .collect::<Vec<_>>();

        let connections = self.connections
            .into_iter()
            .flat_map(|c| conv_undirected_to_directed(c))
            .collect::<Vec<_>>();

        Ok(types::Universe::new(systems.into(), connections.into()))
    }
}