use crate::types;
use crate::types::Navigatable;
use pathfinding::prelude::dijkstra;

pub struct Path<'a> {
    path: Vec<types::SystemId>,
    cur: usize,
    universe: &'a dyn types::Navigatable,
}

impl<'a> Path<'a> {
    pub(self) fn new(universe: &'a dyn types::Navigatable, path: Vec<types::SystemId>) -> Self {
        Self {
            path,
            universe,
            cur: 0,
        }
    }
}

impl<'a> Iterator for Path<'a> {
    type Item = &'a types::System;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.path.len() {
            return None;
        }
        let system_id = &self.path[self.cur];
        self.cur += 1;
        self.universe.get_system(system_id.0)
    }
}

pub struct PathBuilder<'a> {
    universe: &'a dyn types::Navigatable,
    waypoints: Vec<&'a types::System>,
}

impl<'a> PathBuilder<'a> {
    pub fn new(universe: &'a dyn types::Navigatable) -> Self {
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
    pub fn build(self) -> Path<'a> {
        type Cost = u32;
        let u = &self.universe;
        let successor = |id: &types::SystemId| -> Vec<(types::SystemId, Cost)> {
            if let Some(connections) = u.get_connections(id.0) { // -> Vec<Connection>
                connections
                    .iter()
                    .filter_map(|conn| match conn {
                        types::Connection::Bridge(b) => Some((b.to.clone(), 1)),
                        types::Connection::Jump(j) => Some((j.to.clone(), 1)),
                        types::Connection::Wormhole(wh) => Some((wh.to.clone(), 1)),
                    })
                    .collect()
            } else {
                Vec::new()
            }
        };

        let mut result = Vec::new();
        for systems_slice in self.waypoints.windows(2) {
            let a = &systems_slice[0];
            let b = &systems_slice[1];
            // we operate only on system ids
            let (np, _) = dijkstra(&a.id, successor, |id: &types::SystemId| *id == b.id).unwrap();
            result.extend(np);
        }

        Path::new(self.universe, result)
    }
}

#[cfg(feature = "database")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabaseBuilder;
    use std::env;

    extern crate test;

    #[test]
    fn test_dijkstra() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let path = PathBuilder::new(&universe)
            .waypoint(&universe.get_system(30000142).unwrap()) // jita
            .waypoint(&universe.get_system(30000049).unwrap()) // camal
            .build()
            .collect::<Vec<_>>();
        assert_eq!(28, path.len());
        assert_eq!("Jita", path[0].name);
        assert_eq!("Iyen-Oursta", path[2].name);
        assert_eq!("Hek", path[9].name);
        assert_eq!("Camal", path[27].name);
    }

    #[bench]
    fn bench_dijkstra(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&universe)
                .waypoint(&universe.get_system(30000142).unwrap()) // jita
                .waypoint(&universe.get_system(30000049).unwrap()) // camal
                .build()
            );
        });
    }

    #[bench]
    fn bnech_dijkstra_and_collection(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&universe)
                .waypoint(&universe.get_system(30000142).unwrap()) // jita
                .waypoint(&universe.get_system(30000049).unwrap()) // camal
                .build()
                .collect::<Vec<_>>()
            );
        });
    }

    #[test]
    fn test_dijkstra_extended() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let adj = vec![
            types::Connection::Wormhole(types::WormholeConnection {
                from: 30002718.into(), // Rancer
                to: 30000004.into(), // Jark
            })
        ].into();
        let extended = types::ExtendedUniverse::new(&universe, adj);

        let path = PathBuilder::new(&extended)
            .waypoint(&universe.get_system(30000142).unwrap()) // jita
            .waypoint(&universe.get_system(30000049).unwrap()) // camal
            .build()
            .collect::<Vec<_>>();
        assert_eq!(18, path.len());
        assert_eq!("Jita", path[0].name);
        assert_eq!("Iyen-Oursta", path[2].name);
        assert_eq!("Camal", path[17].name);
    }

    #[bench]
    fn bnech_dijkstra_extended(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let adj = vec![
            types::Connection::Wormhole(types::WormholeConnection {
                from: 30002718.into(), // Rancer
                to: 30000004.into(), // Jark
            })
        ].into();
        let extended = types::ExtendedUniverse::new(&universe, adj);
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&extended)
                .waypoint(&universe.get_system(30000142).unwrap()) // jita
                .waypoint(&universe.get_system(30000049).unwrap()) // camal
                .build()
                .collect::<Vec<_>>()
            );
        });
    }
}
