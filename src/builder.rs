/*
 * Copyright (c) 2021. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */
use crate::types;

/// Build a universe from a list of systems and connections.
///`
/// # Example
/// ```
/// use neweden::{
///     Coordinate,
///     Connection,
///     ConnectionType,
///     Security,
///     System,
///     SystemId,
///     StargateType,
///     UniverseBuilder
/// };
///
/// let mut builder = UniverseBuilder::new();
/// builder
///    .system(
///        System {
///            id: SystemId(1),
///            name: "A".to_string(),
///            coordinate: Coordinate { x: 0.0, y: 0.0, z: 0.0 },
///            security: Security(1.0)}
///            )
///    .system(
///        System {
///            id: SystemId(2),
///            name: "B".to_string(),
///            coordinate: Coordinate { x: 0.0, y: 0.0, z: 0.0 },
///            security: Security(0.5)}
///            )
///    .connection(
///        Connection {
///            from: SystemId(1),
///            to: SystemId(2),
///            type_: ConnectionType::Stargate(StargateType::Local) }
///            )
///    .build();
/// ```
pub struct UniverseBuilder {
    systems: types::SystemMap,
    connections: types::AdjacentMap,
}

impl UniverseBuilder {
    pub fn new() -> Self {
        Self {
            systems: types::SystemMap::empty(),
            connections: types::AdjacentMap::empty(),
        }
    }
    /// use neweden::UniverseBuilder;

    pub fn system(mut self, system: types::System) -> Self {
        self.systems.0.insert(system.id, system);
        self
    }

    pub fn connection(mut self, connection: types::Connection) -> Self {
        self.connections.0
            .entry(connection.from)
            .or_insert_with(Vec::new)
            .push(connection);
        self
    }

    pub fn build(self) -> types::Universe {
        types::Universe::new(
            self.systems    ,
            self.connections,
        )
    }
}

/// Extends an existing universe with additional connections.
///`
/// # Example
/// ```rust
/// use neweden::{
///     Coordinate,
///     Connection,
///     ConnectionType,
///     ExtendedUniverseBuilder,
///     Security,
///     System,
///     SystemId,
///     StargateType,
///     UniverseBuilder,
///     WormholeType,
/// };
///
/// let mut builder = UniverseBuilder::new();
/// // Build the universe.
/// let universe = builder
///    .system(
///        System {
///            id: SystemId(1),
///            name: "A".to_string(),
///            coordinate: Coordinate { x: 0.0, y: 0.0, z: 0.0 },
///            security: Security(1.0)
///        }
///     )
///    .system(
///        System {
///            id: SystemId(2),
///            name: "B".to_string(),
///            coordinate: Coordinate { x: 0.0, y: 0.0, z: 0.0 },
///            security: Security(0.5)
///        }
///     )
///    .system(
///        System {
///            id: SystemId(3),
///            name: "C".to_string(),
///            coordinate: Coordinate { x: 0.0, y: 0.0, z: 0.0 },
///            security: Security(0.0)
///        }
///     )
///    .connection(
///        Connection {
///            from: SystemId(1),
///            to: SystemId(2),
///            type_: ConnectionType::Stargate(StargateType::Local) 
///        }
///    )
///    .connection(
///        Connection {
///            from: SystemId(2),
///            to: SystemId(3),
///            type_: ConnectionType::Stargate(StargateType::Local)
///        }
///    )
///    .build();
/// // Extend the universe with a new system.
/// let extended = ExtendedUniverseBuilder::new(&universe)
///     .connection(
///        Connection {
///            from: SystemId(1),
///            to: SystemId(3),
///            type_: ConnectionType::Wormhole(WormholeType::VeryLarge),
///        }
///     )
///     .build();
/// ```
pub struct ExtendedUniverseBuilder<'a, U> {
    universe: &'a U,
    connections: types::AdjacentMap,
}

impl<'a, U: types::Universish + types::Navigatable> ExtendedUniverseBuilder<'a, U> {
    pub fn new(universe: &'a U) -> Self {
        Self {
            universe,
            connections: types::AdjacentMap::empty(),
        }
    }

    /// A convenient way to include a bridge into the universe. While this exposes underlying
    /// mechanics of EVE Online, it makes it is a common enough use case that we include it here.
    pub fn bridge(mut self, location: types::SystemId, type_: types::BridgeType) -> Self {
        let ly: types::Lightyears = type_.clone().into();
        for end in self.universe.get_systems_by_range(&location, ly.into()).unwrap_or(vec![]) {
            let connection = types::Connection {
                from: location,
                to: end.id,
                type_: types::ConnectionType::Bridge(type_.clone())
            };
            self = self.connection(connection);
        }

        self
    }

    pub fn connection(mut self, connection: types::Connection) -> Self {
        self.connections.0
            .entry(connection.from)
            .or_insert_with(Vec::new)
            .push(connection);
        self
    }

    pub fn build(self) -> types::ExtendedUniverse<'a, U> {
        types::ExtendedUniverse::new(
            self.universe,
            self.connections,
        )
    }
}
