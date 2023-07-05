/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */

use pathfinding::prelude::dijkstra;

use crate::types;

#[derive(PartialEq)]
enum PathElementInternal {
    Waypoint(types::SystemId),
    System(types::SystemId),
    Connection(types::ConnectionType),
}

pub enum PathElement<'a> {
    Waypoint(&'a types::System),
    System(&'a types::System),
    Connection(types::ConnectionType),
}

pub struct Path<'a> {
    cur: usize,
    jump_count: usize,
    path: Vec<PathElementInternal>,
    universe: &'a dyn types::Navigatable,
    waypoints: Vec<&'a types::System>,
}

impl<'a> Path<'a> {
    pub(self) fn new(
        universe: &'a dyn types::Navigatable,
        waypoints: Vec<&'a types::System>,
        path: Vec<PathElementInternal>,
        jump_count: usize,
    ) -> Self {
        Self {
            cur: 0,
            jump_count,
            path,
            universe,
            waypoints,
        }
    }

    pub fn jumps(&self) -> usize {
        self.jump_count
    }

    pub fn from(&self) -> Option<&'a types::System> {
        let id = self.path.get(0)?;
        match id {
            PathElementInternal::Connection(_) => None,
            PathElementInternal::System(id) => Some(self.universe.get_system(&id).unwrap()),
            PathElementInternal::Waypoint(id) => Some(self.universe.get_system(&id).unwrap()),
        }
    }

    pub fn to(&self) -> Option<&'a types::System> {
        let id = self.path.get(self.path.len() - 1)?;
        match id {
            PathElementInternal::Connection(_) => None,
            PathElementInternal::System(id) => Some(self.universe.get_system(&id).unwrap()),
            PathElementInternal::Waypoint(id) => Some(self.universe.get_system(&id).unwrap()),
        }
    }

    pub fn iter(&self) -> PathIterator {
        self.into_iter()
    }
    pub fn systems(&self) -> impl Iterator<Item = &types::System> {
        self.iter().filter_map(|f| match f {
            PathElement::System(s) => Some(s),
            PathElement::Waypoint(s) => Some(s),
            _ => None,
        })
    }
}

pub struct PathIterator<'a> {
    cur: usize,
    path: &'a Path<'a>,
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = PathElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.path.path.len() {
            return None;
        }
        let res = match &self.path.path[self.cur] {
            PathElementInternal::Waypoint(id) => {
                PathElement::Waypoint(self.path.universe.get_system(&id).unwrap())
            }
            PathElementInternal::System(id) => {
                PathElement::System(self.path.universe.get_system(&id).unwrap())
            }
            PathElementInternal::Connection(type_) => PathElement::Connection(type_.clone()),
        };
        self.cur += 1;
        Some(res)
    }
}

impl<'a> IntoIterator for &'a Path<'a> {
    type Item = PathElement<'a>;
    type IntoIter = PathIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PathIterator { cur: 0, path: self }
    }
}

impl<'a> Iterator for Path<'a> {
    type Item = PathElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.path.len() {
            return None;
        }
        let res = match &self.path[self.cur] {
            PathElementInternal::Waypoint(id) => {
                PathElement::Waypoint(self.universe.get_system(&id).unwrap())
            }
            PathElementInternal::System(id) => {
                PathElement::System(self.universe.get_system(&id).unwrap())
            }
            PathElementInternal::Connection(type_) => PathElement::Connection(type_.clone()),
        };
        self.cur += 1;
        Some(res)
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
                match universe.get_system(&to).unwrap().security.into() {
                    types::SecurityClass::Highsec => 1,
                    types::SecurityClass::Lowsec | types::SecurityClass::Nullsec => 1000,
                }
            }
            Self::LowsecAndNullsec => match universe.get_system(&to).unwrap().security.into() {
                types::SecurityClass::Highsec => 1000,
                types::SecurityClass::Lowsec | types::SecurityClass::Nullsec => 1,
            },
        }
    }
}

#[derive(Eq, Clone)]
struct Succ {
    id: types::SystemId,
    via: Option<types::ConnectionType>,
}

impl std::hash::Hash for Succ {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::cmp::PartialEq for Succ {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
            waypoints: vec![],
            preference: Preference::Shortest,
        }
    }

    pub fn waypoint(mut self, system: &'a types::System) -> Self {
        self.waypoints.push(system);
        self
    }

    pub fn waypoints(mut self, systems: Vec<&'a types::System>) -> Self {
        self.waypoints.extend(systems);
        self
    }

    pub fn prefer(mut self, preference: Preference) -> Self {
        self.preference = preference;
        self
    }

    // TODO: We need to include the Connection itself, otherwise connections can be
    // ambiguous in the rare case that a wormhole leads to the same system next door.
    // In practise it likely doesn't matter.
    pub fn build(self) -> Option<Path<'a>> {
        let successor = |s: &Succ| -> Vec<(Succ, Cost)> {
            if let Some(connections) = self.universe.get_connections(&s.id) {
                connections
                    .iter()
                    .filter_map(|conn| {
                        let cost = self.preference.cost(self.universe, conn.to);
                        let succ = Succ {
                            id: conn.to,
                            via: Some(conn.type_.clone()),
                        };
                        Some((succ, cost))
                    })
                    .collect()
            } else {
                Vec::new()
            }
        };

        let mut jump_count = 0;
        let mut result = Vec::new();
        for systems_slice in self.waypoints.windows(2) {
            let a = &systems_slice[0];
            let b = &systems_slice[1];
            // we operate only on system ids
            if let Some((np, _)) = dijkstra(
                &Succ {
                    id: a.id,
                    via: None,
                },
                successor,
                |s: &Succ| s.id == b.id,
            ) {
                for succ in np {
                    if let Some(via) = succ.via {
                        result.push(PathElementInternal::Connection(via));
                        jump_count += 1;
                    }
                    if succ.id == a.id || succ.id == b.id {
                        result.push(PathElementInternal::Waypoint(succ.id));
                    } else {
                        result.push(PathElementInternal::System(succ.id));
                    }
                }
            } else {
                return None;
            }
        }

        result.dedup();
        Some(Path::new(self.universe, self.waypoints, result, jump_count))
    }
}

#[cfg(feature = "sqlite")]
#[cfg(test)]
mod tests {
    use std::env;

    use crate::source::sqlite::DatabaseBuilder;
    use crate::types::Navigatable;

    use super::*;

    extern crate test;

    fn name<'a>(e: &PathElement<'a>) -> &'a str {
        match e {
            PathElement::System(sys) => &sys.name,
            _ => panic!("should not be reached"),
        }
    }

    #[test]
    fn test_dijkstra() {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            let path = PathBuilder::new(&universe)
                .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                .build()
                .unwrap();
            assert_eq!(27, path.jumps());

            let sys = path.systems().collect::<Vec<_>>();
            assert_eq!("Jita", sys[0].name);
            assert_eq!("Iyen-Oursta", sys[2].name);
            assert_eq!("Hek", sys[9].name);
            assert_eq!("Camal", sys[27].name);
        }
    }

    #[test]
    fn test_dijkstra_preference_safer() {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            let path = PathBuilder::new(&universe)
                .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                .prefer(Preference::Highsec)
                .build()
                .unwrap();
            assert_eq!(37, path.jumps());

            let sys = path.systems().collect::<Vec<_>>();
            assert_eq!("Jita", sys[0].name);
            assert_eq!("Urlen", sys[2].name);
            assert_eq!("Trytedald", sys[25].name);
            assert_eq!("Camal", sys[37].name);
        }
    }

    #[test]
    fn test_dijkstra_preference_unsafer() {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            let path = PathBuilder::new(&universe)
                .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                .prefer(Preference::LowsecAndNullsec)
                .build()
                .unwrap();
            assert_eq!(69, path.jumps());

            let sys = path.systems().collect::<Vec<_>>();
            assert_eq!("Jita", sys[0].name);
            assert_eq!("LXQ2-T", sys[39].name);
            assert_eq!("6-EQYE", sys[48].name);
            assert_eq!("Camal", sys[69].name);
        }
    }

    #[bench]
    fn bench_dijkstra(b: &mut test::Bencher) {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            b.iter(|| {
                test::black_box(
                    PathBuilder::new(&universe)
                        .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                        .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                        .build()
                        .unwrap(),
                );
            });
        }
    }

    #[bench]
    fn bnech_dijkstra_and_collection(b: &mut test::Bencher) {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            b.iter(|| {
                test::black_box(
                    PathBuilder::new(&universe)
                        .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                        .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                        .build()
                        .unwrap()
                        .collect::<Vec<_>>(),
                );
            });
        }
    }

    #[bench]
    fn bnech_dijkstra_preference_highsec(b: &mut test::Bencher) {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            b.iter(|| {
                test::black_box(
                    PathBuilder::new(&universe)
                        .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                        .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                        .prefer(Preference::Highsec)
                        .build()
                        .unwrap()
                        .collect::<Vec<_>>(),
                );
            });
        }
    }

    #[bench]
    fn bnech_dijkstra_preference_lowsec(b: &mut test::Bencher) {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            b.iter(|| {
                test::black_box(
                    PathBuilder::new(&universe)
                        .waypoint(&universe.get_system(&30000142.into()).unwrap()) // jita
                        .waypoint(&universe.get_system(&30000049.into()).unwrap()) // camal
                        .prefer(Preference::LowsecAndNullsec)
                        .build()
                        .unwrap()
                        .collect::<Vec<_>>(),
                );
            });
        }
    }

    #[bench]
    fn bnech_dijkstra_longest(b: &mut test::Bencher) {
        if let Ok(uri) = env::var("SQLITE_URI") {
            let universe = DatabaseBuilder::new(&uri).build().unwrap();
            b.iter(|| {
                test::black_box(
                    PathBuilder::new(&universe)
                        // this is the longest direct route in eve, 99 jumps
                        .waypoint(&universe.get_system(&30001947.into()).unwrap()) // 373Z-7
                        .waypoint(&universe.get_system(&30004377.into()).unwrap()) // SVB-RE
                        .build()
                        .unwrap()
                        .collect::<Vec<_>>(),
                );
            });
        }
    }

    #[test]
    fn test_dijkstra_extended() {
        if let Ok(uri) = env::var("SQLITE_URI") {
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
                .unwrap();

            assert_eq!(17, path.jumps());

            let sys = path.systems().collect::<Vec<_>>();
            assert_eq!("Jita", sys[0].name);
            assert_eq!("Iyen-Oursta", sys[2].name);
            assert_eq!("Camal", sys[17].name);
        }
    }

    #[bench]
    fn bnech_dijkstra_extended(b: &mut test::Bencher) {
        if let Ok(uri) = env::var("SQLITE_URI") {
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
                        .unwrap()
                        .collect::<Vec<_>>(),
                );
            });
        }
    }
}
