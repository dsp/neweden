use crate::types;

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
