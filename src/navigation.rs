/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */

use pathfinding::prelude::dijkstra;

use crate::types;

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
        self.universe.get_system(&system_id)
    }
}

type Cost = u32;

pub enum Preference {
    Shortest,
    Highsec,
    LowsecAndNullsec,
}

impl Preference {
    fn cost(&self, universe: &dyn types::Navigatable, to: types::SystemId) -> Cost {
        match self {
            Self::Shortest => 1, // all are equal distance
            Self::Highsec => {
                // we must have positive weights
                // security can go from -1.0 to 1.0
                match universe.get_system(&to).unwrap().security.clone().into() {
                    types::SecurityClass::Highsec => 1,
                    types::SecurityClass::Lowsec | types::SecurityClass::Nullsec => 1000,
                }
            }
            Self::LowsecAndNullsec => {
                match universe.get_system(&to).unwrap().security.clone().into() {
                    types::SecurityClass::Highsec => 1000,
                    types::SecurityClass::Lowsec | types::SecurityClass::Nullsec => 1,
                }
            }
        }
    }
}

pub struct PathBuilder<'a> {
    universe: &'a dyn types::Navigatable,
    waypoints: Vec<&'a types::System>,
    preference: Preference,
}

impl<'a> PathBuilder<'a> {
    pub fn new(universe: &'a dyn types::Navigatable) -> Self {
        Self {
            universe: universe,
            waypoints: Vec::new(),
            preference: Preference::Shortest,
        }
    }

    pub fn waypoint(mut self, system: &'a types::System) -> Self {
        self.waypoints.push(system);
        self
    }

    pub fn prefer(mut self, preference: Preference) -> Self {
        self.preference = preference;
        self
    }

    // TODO: We need to include the Connection itself, otherwise connections can be
    // ambiguous in the rare case that a wormhole leads to the same system next door.
    // In practise it likely doesn't matter.
    pub fn build(self) -> Path<'a> {
        let successor = |id: &types::SystemId| -> Vec<(types::SystemId, Cost)> {
            if let Some(connections) = self.universe.get_connections(&id) {
                connections
                    .iter()
                    .filter_map(|conn| {
                        let cost = self.preference.cost(self.universe, conn.to.clone());
                        Some((conn.to.clone(), cost))
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
    use std::env;

    use crate::source::database::DatabaseBuilder;
    use crate::types::Navigatable;

    use super::*;

    extern crate test;

    #[test]
    fn test_dijkstra() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let path = PathBuilder::new(&universe)
            .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
            .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
            .build()
            .collect::<Vec<_>>();
        assert_eq!(28, path.len());
        assert_eq!("Jita", path[0].name);
        assert_eq!("Iyen-Oursta", path[2].name);
        assert_eq!("Hek", path[9].name);
        assert_eq!("Camal", path[27].name);
    }

    #[test]
    fn test_dijkstra_preference_safer() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let path = PathBuilder::new(&universe)
            .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
            .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
            .prefer(Preference::Highsec)
            .build()
            .collect::<Vec<_>>();
        assert_eq!(36, path.len());
        assert_eq!("Jita", path[0].name);
        assert_eq!("Urlen", path[2].name);
        assert_eq!("Lashesih", path[25].name);
        assert_eq!("Camal", path[35].name);
    }

    #[test]
    fn test_dijkstra_preference_unsafer() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let path = PathBuilder::new(&universe)
            .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
            .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
            .prefer(Preference::LowsecAndNullsec)
            .build()
            .collect::<Vec<_>>();
        assert_eq!(70, path.len());
        assert_eq!("Jita", path[0].name);
        assert_eq!("LXQ2-T", path[39].name);
        assert_eq!("6-EQYE", path[48].name);
        assert_eq!("Camal", path[69].name);
    }

    #[bench]
    fn bench_dijkstra(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&universe)
                    .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                    .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                    .build(),
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
                    .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                    .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                    .build()
                    .collect::<Vec<_>>(),
            );
        });
    }

    #[bench]
    fn bnech_dijkstra_preference_highsec(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&universe)
                    .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                    .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                    .prefer(Preference::Highsec)
                    .build()
                    .collect::<Vec<_>>(),
            );
        });
    }

    #[bench]
    fn bnech_dijkstra_preference_lowsec(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&universe)
                    .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                    .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                    .prefer(Preference::LowsecAndNullsec)
                    .build()
                    .collect::<Vec<_>>(),
            );
        });
    }

    #[bench]
    fn bnech_dijkstra_longest(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&universe)
                    // this is the longest direct route in eve, 99 jumps
                    .waypoint(&universe.get_system(&30001947.into()).unwrap()) // 373Z-7
                    .waypoint(&universe.get_system(&30004377.into()).unwrap()) // SVB-RE
                    .build()
                    .collect::<Vec<_>>(),
            );
        });
    }

    #[test]
    fn test_dijkstra_extended() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let adj = vec![types::Connection {
            from: 30002718.into(), // Rancer
            to: 30000004.into(),   // Jark
            type_: types::ConnectionType::Wormhole(types::WormholeType::VeryLarge),
        }]
        .into();
        let extended = types::ExtendedUniverse::new(&universe, adj);

        let path = PathBuilder::new(&extended)
            .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
            .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
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
        let adj = vec![types::Connection {
            from: 30002718.into(), // Rancer
            to: 30000004.into(),   // Jark
            type_: types::ConnectionType::Wormhole(types::WormholeType::VeryLarge),
        }]
        .into();
        let extended = types::ExtendedUniverse::new(&universe, adj);
        b.iter(|| {
            test::black_box(
                PathBuilder::new(&extended)
                    .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                    .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                    .build()
                    .collect::<Vec<_>>(),
            );
        });
    }
}
