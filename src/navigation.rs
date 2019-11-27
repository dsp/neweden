use crate::types;
use pathfinding::prelude::dijkstra;

pub struct Path<'a> {
    path: Vec<types::SystemId>,
    cur: usize,
    universe: &'a types::Universe,
}

impl<'a> Path<'a> {
    pub(self) fn new(universe: &'a types::Universe, path: Vec<types::SystemId>) -> Self {
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
        Some(&self.universe.systems[&system_id])
    }
}

pub struct PathBuilder<'a> {
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
    pub fn build(self) -> Path<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabaseBuilder;
    use std::env;

    extern crate test;

    #[test]
    fn test_me() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let path = PathBuilder::new(&universe)
            .waypoint(&universe.systems[&types::SystemId(30000142)]) // jita
            .waypoint(&universe.systems[&types::SystemId(30000049)]) // camal
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
            test::black_box(PathBuilder::new(&universe)
                .waypoint(&universe.systems[&types::SystemId(30000142)]) // jita
                .waypoint(&universe.systems[&types::SystemId(30000049)]) // camal
                .build());
        });
    }

    #[bench]
    fn bnech_dijkstra_and_collection(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        b.iter(|| {
            test::black_box(PathBuilder::new(&universe)
                .waypoint(&universe.systems[&types::SystemId(30000142)]) // jita
                .waypoint(&universe.systems[&types::SystemId(30000049)]) // camal
                .build()
                .collect::<Vec<_>>());
        });
    }
}
