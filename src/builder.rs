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
            self.systems,
            self.connections,
        )
    }
}

