use crate::types;
use pathfinding::prelude::dijkstra;

pub type Path = Vec<types::SystemId>;

struct PathBuilder<'a> {
    universe: &'a types::Universe,
    waypoints: Vec<&'a types::System>,
}

impl<'a> PathBuilder<'a> {
    pub fn new(universe: &'a types::Universe) -> Self {
        Self {
            universe: universe,
            waypoints: Vec::new(),
        }
    }

    pub fn waypoint(mut self, system: &'a types::System) -> Self {
        self.waypoints.push(system);
        self
    }

    pub fn using(/* mut */ self) -> Self {
        unimplemented!()
    }

    // TODO: We need to include the Connection itself, otherwise connections can be
    // ambiguous in the rare case that a wormhole leads to the same system next door.
    // In practise it likely doesn't matter.
    pub fn build(self) -> Option<Path> {
        type Cost = u32;
        // fn successor(system: &types::System) -> Vec<types::System> {
        let u = &self.universe;
        let successor = |id: &types::SystemId| -> Vec<(types::SystemId, Cost)> {
            u.connections[&id] // -> Vec<Connection>
                .iter()
                .filter_map(|conn| match conn {
                    types::Connection::Jump(sc) => Some((sc.to.clone(), 1)),
                    _ => None,
                })
                .collect()
        };

        if self.waypoints.len() < 2 {
            return None;
        }

        let mut p = Vec::new();
        for sl in self.waypoints.windows(2) {
            let a = &sl[0];
            let b = &sl[1];
            let (np, _) = dijkstra(&a.id, successor, |id: &types::SystemId| *id == b.id).unwrap();
            p.extend(np);
        }

        None
        // Some(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::DatabaseBuilder;
    use std::env;

    #[test]
    fn test_me() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build();
        //     .with_connections(vec![Connection])
    }
}
