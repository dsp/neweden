/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */

use rstar;
use std::collections::HashMap;
use pathfinding::undirected::connected_components::connected_components;

/// Describes the ID of a solar system. Can be casted to from i32 or u32 using .into()
///
/// # Example
/// ```
/// use neweden::SystemId;
///
/// let system_id: SystemId = 30000142.into(); // returns a SystemId
/// assert_eq!(system_id, SystemId(30000142));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemId(pub u32);

impl From<u32> for SystemId {
    fn from(other: u32) -> Self {
        SystemId(other)
    }
}

impl From<i32> for SystemId {
    fn from(other: i32) -> Self {
        SystemId(other as u32)
    }
}

/// Describes a security rating. A security rating is between -1.0 and 1.0.
#[derive(Debug, Clone, PartialEq)]
pub struct Security(pub f32); // TODO Bound check

impl From<f32> for Security {
    fn from(other: f32) -> Self {
        Security(other)
    }
}

/// Describes if a system's security rating is considered Highsec, Lowsec or Nullsec.
/// In Eve Online, 1.0 t 0.45 is considered highsec. 0.0 to 0.45 is considered lowsec,
/// and below 0.0 is considered nullsec.
///
/// A security instance can be converted into a SecurityClass.
///
/// # Example
/// ```
/// use neweden::{Security, SecurityClass};
/// let s1 = Security(0.443);
/// let sc1: SecurityClass = s1.into();
/// assert_eq!(sc1, SecurityClass::Lowsec);
/// let s2 = Security(-0.24);
/// assert_eq!(SecurityClass::from(s2), SecurityClass::Nullsec);
/// let s3 = Security(0.74);
/// assert_eq!(SecurityClass::from(s3), SecurityClass::Highsec);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityClass {
    Highsec,
    Lowsec,
    Nullsec,
}

impl From<&Security> for SecurityClass {
    fn from(other: &Security) -> Self {
        let sec = (other.0 * 10.0).round() / 10.0;
        if sec < 0.0 {
            Self::Nullsec
        } else if sec < 0.5 {
            Self::Lowsec
        } else {
            Self::Highsec
        }
    }
}

impl From<Security> for SecurityClass {
    fn from(other: Security) -> Self {
        let sec = (other.0 * 10.0).round() / 10.0;
        if sec < 0.0 {
            Self::Nullsec
        } else if sec < 0.5 {
            Self::Lowsec
        } else {
            Self::Highsec
        }
    }
}

/// Defines a connection between two systems.
#[derive(Debug)]
pub struct Connection {
    pub from: SystemId,
    pub to: SystemId,
    pub type_: ConnectionType,
}

/// The type of connection between two systems.
/// Can be a bridge, a stargate or a wormhole.
#[derive(Debug)]
pub enum ConnectionType {
    Stargate(StargateType),
    Bridge(BridgeType),
    Wormhole(WormholeType),
}

// Information about a bridge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeType {
    // TODO: introduce a type JumpDrive
    Titan(u8, u8),    // jump drive calibration, jump fuel conservation
    BlackOps(u8, u8), // jump drive calibration, jump fuel conservation
}

/// Information about a stargate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StargateType {
    Local,
    Constellation,
    Regional,
}

/// Information about a wormhole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WormholeType {
    VeryLarge, // everything, except supers+
    Large,     // battleships
    Medium,    // battlecruisers, etc
    Small,     // frigates, etc
}

/// Defines a system class. A system is either part of
/// the known space (SystemClass::KSpace) or wormhole space
/// (SystemClass::WSpace).
///
/// A System reference can be casted into this.
/// # Example
/// ```
/// use neweden::{System, Coordinate, SystemClass};
/// let jita = System {
///     id: 30000142.into(),
///     name: "Jita".to_string(),
///     coordinate: Coordinate {
///         x: -1.2906e+17_f64,
///         y: 6.07553e+16_f64,
///         z: 1.17469e+17_f64,
///     },
///     security: 0.9459.into(),
/// };
/// assert_eq!(SystemClass::from(&jita), SystemClass::KSpace);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemClass {
    KSpace,
    WSpace,
}

impl From<System> for SystemClass {
    fn from(s: System) -> Self {
        match s.id {
            SystemId(0..=30999999) => Self::KSpace,
            SystemId(31000000..=31999999) => Self::WSpace,
            _ => panic!("unknown space."),
        }
    }
}

impl From<&System> for SystemClass {
    fn from(s: &System) -> Self {
        match s.id {
            SystemId(0..=30999999) => Self::KSpace,
            SystemId(31000000..=31999999) => Self::WSpace,
            _ => panic!("unknown space."),
        }
    }
}

/// Describes the coordinate of a system in Eve Online.
#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Describe a system.
#[derive(Debug, Clone)]
pub struct System {
    // The ID of a system. Coorespondes to the field mapSolarSystems.solarSystemID in the SDE.
    pub id: SystemId,
    // The name of a system. Coorespondes to the field mapSolarSystems.solarSystemName in the SDE.
    pub name: String,
    // The coordinate of a the system in the universe.
    pub coordinate: Coordinate,
    // The security rating of the system.
    pub security: Security,
}

impl std::cmp::Eq for System {}
impl std::cmp::PartialEq for System {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for System {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
struct Celestial {}

#[derive(Debug)]
pub struct SystemMap(HashMap<SystemId, System>);

impl From<Vec<System>> for SystemMap {
    fn from(systems: Vec<System>) -> Self {
        let mut system_map = HashMap::new();
        for system in systems {
            system_map.insert(system.id.clone(), system);
        }

        Self(system_map)
    }
}

#[derive(Debug)]
pub struct AdjacentMap(HashMap<SystemId, Vec<Connection>>);

impl From<Vec<Connection>> for AdjacentMap {
    fn from(connections: Vec<Connection>) -> Self {
        let mut adjacent_map = HashMap::new();
        for connection in connections {
            adjacent_map
                .entry(connection.from.clone())
                .or_insert_with(Vec::new)
                .push(connection);
        }

        Self(adjacent_map)
    }
}

// TODO: Implement conversions between those

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Lightyears(pub f64);
impl From<Lightyears> for Meters {
    fn from(other: Lightyears) -> Self{
        const LY_IN_KM: f64 = 9_460_730_472_580.8;
        Meters(other.0 * LY_IN_KM * 1_000.0)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Au(pub f64);
impl From<Au> for Meters {
    fn from(other: Au) -> Self {
        const AU_TO_KM: f64 = 149_597_871.0;
        Meters(other.0 * AU_TO_KM * 1_000.0)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Kilometers(pub f64);
impl From<Kilometers> for Meters {
    fn from(other: Kilometers) -> Self{
        Meters(other.0 * 1_000.0)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Meters(pub f64);

/// Describes universes that are navigatable. Only navigatable universes can be used
/// for pathfinding. Two main implementation exists: `Universe` and `ExtendedUniverse`.
pub trait Navigatable {
    fn get_system<'a>(&self, id: SystemId) -> Option<&System>;
    fn get_connections<'a>(&self, from: SystemId) -> Option<&Vec<Connection>>;
    fn get_systems_by_range<'a>(&self, from: SystemId, range: Meters) -> Option<Vec<&System>>;
}

/// Describes the known systesms and their connections in new eden universe.
/// `Universe` implements `Navigatable` and can be used in pathfinding.
///
/// `Universe` is intended to be used immutable and can only be instantiated
/// from a data source such as a database. If you need to add additional connections,
/// such as dynamic wormhole connections during pathfinding, construct an `ExtendedUniverse`
/// from a universe by calling `.extend()` or `ExtendedUniverse::new()`.
///
/// # Example
/// ```
/// use std::env;
/// use neweden::database::DatabaseBuilder;
/// use neweden::Navigatable;
///
/// let uri = std::env::var("DATABASE_URL").unwrap();
/// let universe = DatabaseBuilder::new(&uri).build().unwrap();
/// let system_id = 30000142.into(); // returns a SystemId
/// println!("{:?}", universe.get_system(system_id).unwrap().name); // Jita
/// ```
#[derive(Debug)]
pub struct Universe {
    systems: SystemMap,
    connections: AdjacentMap,
    rtree: rstar::RTree<System>,
}

impl System {
    fn to_point(&self) -> [f64; 3] {
        [self.coordinate.x, self.coordinate.y, self.coordinate.z]
    }
    fn distance(&self, point: &[f64; 3]) -> Meters
    {
        let d_x = self.coordinate.x - point[0];
        let d_y = self.coordinate.y - point[1];
        let d_z = self.coordinate.z - point[2];
        let distance = (d_x * d_x + d_y * d_y + d_z * d_z).sqrt();
        // We must return the squared distance!
        Meters(distance)
    }
}

impl rstar::RTreeObject for System {
    type Envelope = rstar::AABB<[f64; 3]>;

    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point(self.to_point())
    }
}

impl rstar::PointDistance for System {
    fn distance_2(&self, point: &[f64; 3]) -> f64
    {
        let d_x = self.coordinate.x - point[0];
        let d_y = self.coordinate.y - point[1];
        let d_z = self.coordinate.z - point[2];
        let distance = (d_x * d_x + d_y * d_y + d_z * d_z).sqrt();
        // We must return the squared distance!
        distance * distance
    }
}

impl Universe {
    /// Create a new universe. This is internal to the crate as only a data source
    /// is allowed to create it.
    pub(crate) fn new(systems: SystemMap, connections: AdjacentMap) -> Self {
        // TODO: Remove the clone and use references into the map if possible
        let spatial_data = systems.0.values().map(|s| s.clone()).collect::<Vec<_>>();

        Self {
            systems,
            connections,
            rtree: rstar::RTree::bulk_load(spatial_data),
        }
    }

    pub fn extend(&self, connections: AdjacentMap) -> ExtendedUniverse {
        ExtendedUniverse::new(self, connections)
    }

    pub fn systems(&self) -> Vec<&System> {
        self.systems.0.values().collect::<Vec<&System>>()
    }

    pub fn connections(&self) -> Vec<(SystemId, SystemId)> {
        let mut connections = Vec::new();
        for adjacent in self.connections.0.values() {
            for conn in adjacent {
               connections.push((conn.from.clone(), conn.to.clone()))
            }
        }
        connections
    }
}

impl Navigatable for Universe {
    fn get_system<'a>(&self, id: SystemId) -> Option<&System> {
        self.systems.0.get(&id)
    }

    fn get_connections<'a>(&self, from: SystemId) -> Option<&Vec<Connection>> {
        self.connections.0.get(&from)
    }

    fn get_systems_by_range<'a>(&self, from: SystemId, range: Meters) -> Option<Vec<&System>> {
        // it is very important that we use KM, since all distances in the database are in KM, because CCP.
        let system = self.get_system(from)?;
        let systems = self.rtree
            .locate_within_distance(system.to_point(), range.0 * range.0)
            .collect::<Vec<_>>();
        Some(systems)
    }
}

/// Extends the universe with dynamic connections. This is intended to be used
/// to allow pathfinding through wormholes and titan bridges.
///
/// # Example
/// ```
/// use std::env;
/// use neweden::database::DatabaseBuilder;
/// use neweden::navigation::PathBuilder;
/// use neweden::Navigatable;
/// use neweden::{Connection, ConnectionType, WormholeType};
///
/// let uri = std::env::var("DATABASE_URL").unwrap();
/// let wormholes = vec![Connection {
///     from: 30002718.into(), // Rancer
///     to: 30000049.into(),  // Camal
///     type_: ConnectionType::Wormhole(WormholeType::VeryLarge),
/// }];
/// let universe = DatabaseBuilder::new(&uri).build().unwrap();
/// let extended = universe.extend(wormholes.into()); // make into an adjacent map and pass into extend()
/// let path = PathBuilder::new(&extended)
///     .waypoint(extended.get_system(30002718.into()).unwrap()) // from Rancer
///     .waypoint(extended.get_system(30000049.into()).unwrap()) // to Camal
///     .build() // returns an iterator
///     .collect::<Vec<_>>();
/// assert_eq!(2, path.len()); // direct jump through our wormhole
/// ```
#[derive(Debug)]
pub struct ExtendedUniverse<'a> {
    universe: &'a Universe,
    connections: AdjacentMap,
}

impl<'a> ExtendedUniverse<'a> {
    pub fn new(universe: &'a Universe, connections: AdjacentMap) -> Self {
        Self {
            universe,
            connections,
        }
    }
}

impl<'b> Navigatable for ExtendedUniverse<'b> {
    fn get_system<'a>(&self, id: SystemId) -> Option<&System> {
        self.universe.get_system(id)
    }

    fn get_connections<'a>(&self, from: SystemId) -> Option<&Vec<Connection>> {
        self.connections
            .0
            .get(&from)
            .or(self.universe.get_connections(from))
    }

    fn get_systems_by_range<'a>(&self, from: SystemId, range: Meters) -> Option<Vec<&System>> {
        self.universe.get_systems_by_range(from, range)
    }
}

#[cfg(all(test, feature = "database"))]
mod tests {
    use std::env;

    use crate::database::DatabaseBuilder;
    use crate::rules;

    use super::*;

    extern crate test;

    #[test]
    fn test_range_query() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let camal_id = 30000049.into();
        // let faspera_id = 30000044.into();
        let systems = universe.get_systems_by_range(camal_id, Lightyears(7.0).into()).unwrap();
        let jumpable = systems.into_iter().filter(|x| rules::allows_cynos(x)).collect::<Vec<_>>();
        assert_eq!(115, jumpable.len());
    }

    #[bench]
    fn bench_range_query(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let camal_id: SystemId = 30000049.into();
        // let faspera_id = 30000044.into();
        b.iter(move || {
            test::black_box(
                universe.get_systems_by_range(camal_id.clone(), Lightyears(7.0).into()));
        });
        // let jumpable = systems.into_iter().filter(|x| rules::allows_cynos(x)).collect::<Vec<_>>();
        // assert_eq!(115, jumpable.len());
    }
}