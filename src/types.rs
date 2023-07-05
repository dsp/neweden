/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */
use rstar;
use std::collections::HashMap;

/// Describes the ID of a solar system. Can be casted to from i32 or u32 using .into()
///
/// # Example
/// ```
/// use neweden::SystemId;
///
/// let system_id: SystemId = 30000142.into(); // returns a SystemId
/// assert_eq!(system_id, SystemId(30000142));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
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
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
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
#[derive(Debug, Clone)]
pub struct Connection {
    pub from: SystemId,
    pub to: SystemId,
    pub type_: ConnectionType,
}

/// The type of connection between two systems.
/// Can be a bridge, a stargate or a wormhole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    Stargate(StargateType),
    Bridge(BridgeType),
    Wormhole(WormholeType),
}

/// The type of bridge. Can be either a titan bridge
/// or a blackops bridge. Provides information about the
/// skill-level used. You can calculate the bridge distance
/// using an `Into` conversion, similar to `JumpdriveShip`.
///
/// # Example
/// ```
/// use neweden::{BridgeType, Lightyears, JumpdriveSkills};
///
/// let titan = BridgeType::Titan(JumpdriveSkills::new(4, 5));
/// let ly: Lightyears = titan.into();
/// println!("titan's bridge range with JDC4 is {:?}", ly);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeType {
    // TODO: introduce a type JumpDrive
    Titan(JumpdriveSkills), // jump drive calibration, jump fuel conservation
    BlackOps(JumpdriveSkills), // jump drive calibration, jump fuel conservation
}

impl std::convert::Into<Lightyears> for BridgeType {
    fn into(self) -> Lightyears {
        match self {
            Self::BlackOps(skills) => skills.range_from_base(Lightyears(4.0)),
            Self::Titan(skills) => skills.range_from_base(Lightyears(3.0)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JumpdriveSkills {
    jump_drive_calibration: u8,
    fuel_conversation: u8,
}

impl JumpdriveSkills {
    pub fn new(jump_drive_calibration: u8, fuel_conversation: u8) -> Self {
        Self {
            jump_drive_calibration,
            fuel_conversation,
        }
    }

    pub fn range_from_base(&self, ly: Lightyears) -> Lightyears {
        let jdc = f64::from(self.jump_drive_calibration);
        ly + (ly * 0.2 * jdc)
    }
}

/// Conversion for jumpdrive capable ships.
/// You can get the jumprange of a ship through Into conversion.
///
/// # Example
/// ```
/// use neweden::{JumpdriveShip, Lightyears, JumpdriveSkills};
///
/// let titan = JumpdriveShip::Titan(JumpdriveSkills::new(5, 5));
/// let ly: Lightyears = titan.into();
/// println!("titan's jump range with JDC5 is {:?}", ly);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JumpdriveShip {
    BlackOps(JumpdriveSkills),
    CapitalIndustrial(JumpdriveSkills),
    Carrier(JumpdriveSkills),
    Dreadnought(JumpdriveSkills),
    ForceAuxiliary(JumpdriveSkills),
    Jumpfreighter(JumpdriveSkills),
    Supercarrier(JumpdriveSkills),
    Titan(JumpdriveSkills),
}

impl std::convert::Into<Lightyears> for JumpdriveShip {
    fn into(self) -> Lightyears {
        match self {
            Self::BlackOps(skills) => skills.range_from_base(Lightyears(4.0)),
            Self::CapitalIndustrial(skills) => skills.range_from_base(Lightyears(5.0)),
            Self::Carrier(skills) => skills.range_from_base(Lightyears(3.5)),
            Self::Dreadnought(skills) => skills.range_from_base(Lightyears(3.5)),
            Self::ForceAuxiliary(skills) => skills.range_from_base(Lightyears(3.5)),
            Self::Jumpfreighter(skills) => skills.range_from_base(Lightyears(5.0)),
            Self::Supercarrier(skills) => skills.range_from_base(Lightyears(3.0)),
            Self::Titan(skills) => skills.range_from_base(Lightyears(3.0)),
        }
    }
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
pub struct SystemMap(pub(crate) HashMap<SystemId, System>);

impl SystemMap {
    pub fn empty() -> Self {
        Self(HashMap::new())
    }
    pub fn get(&self, k: &SystemId) -> Option<&System> {
        self.0.get(k)
    }
}

impl From<Vec<System>> for SystemMap {
    fn from(systems: Vec<System>) -> Self {
        let mut system_map = HashMap::new();
        for system in systems {
            system_map.insert(system.id, system);
        }

        Self(system_map)
    }
}

#[derive(Debug)]
pub struct AdjacentMap(pub(crate) HashMap<SystemId, Vec<Connection>>);

impl AdjacentMap {
    pub fn empty() -> Self {
        Self(HashMap::new())
    }
}
impl From<Vec<Connection>> for AdjacentMap {
    fn from(connections: Vec<Connection>) -> Self {
        let mut adjacent_map = HashMap::new();
        for connection in connections {
            adjacent_map
                .entry(connection.from)
                .or_insert_with(Vec::new)
                .push(connection);
        }

        Self(adjacent_map)
    }
}

// TODO: Implement conversions between those

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Lightyears(pub f64);
impl From<Lightyears> for Meters {
    fn from(other: Lightyears) -> Self {
        const LY_IN_KM: f64 = 9_460_730_472_580.8;
        Meters(other.0 * LY_IN_KM * 1_000.0)
    }
}

impl std::ops::Add for Lightyears {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Lightyears(self.0 + rhs.0)
    }
}

impl std::ops::Mul for Lightyears {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Lightyears(self.0 * rhs.0)
    }
}

impl std::ops::Mul<f64> for Lightyears {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Lightyears(self.0 * rhs)
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Au(pub f64);
impl From<Au> for Meters {
    fn from(other: Au) -> Self {
        const AU_TO_KM: f64 = 149_597_871.0;
        Meters(other.0 * AU_TO_KM * 1_000.0)
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Kilometers(pub f64);
impl From<Kilometers> for Meters {
    fn from(other: Kilometers) -> Self {
        Meters(other.0 * 1_000.0)
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Meters(pub f64);

/// Describes universes that are navigatable. Only navigatable universes can be used
/// for pathfinding. Two main implementation exists: `Universe` and `ExtendedUniverse`.
pub trait Navigatable {
    fn get_system<'a>(&self, id: &SystemId) -> Option<&System>;
    fn get_connections<'a>(&self, from: &SystemId) -> Option<Vec<Connection>>;
    fn get_systems_by_range<'a>(&self, from: &SystemId, range: Meters) -> Option<Vec<&System>>;
}

pub trait Galaxy {
    fn connections(&self) -> Vec<(SystemId, SystemId)>;
    fn systems(&self) -> Vec<&System>;
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
/// use neweden::source::sqlite::DatabaseBuilder;
/// use neweden::Navigatable;
///
/// let uri = std::env::var("SQLITE_URI").unwrap();
/// let universe = DatabaseBuilder::new(&uri).build().unwrap();
/// let system_id = 30000142.into(); // returns a SystemId
///
/// println!("{:?}", universe.get_system(&system_id).unwrap().name); // Jita
/// ```
#[derive(Debug)]
pub struct Universe {
    pub(crate) systems: SystemMap,
    pub(crate) connections: AdjacentMap,
    pub(crate) rtree: rstar::RTree<System>,
}

impl System {
    fn to_point(&self) -> [f64; 3] {
        [self.coordinate.x, self.coordinate.y, self.coordinate.z]
    }

    fn distance(&self, point: &[f64; 3]) -> Meters {
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
    fn distance_2(&self, point: &[f64; 3]) -> f64 {
        let d_x = self.coordinate.x - point[0];
        let d_y = self.coordinate.y - point[1];
        let d_z = self.coordinate.z - point[2];
        let distance = (d_x * d_x + d_y * d_y + d_z * d_z).sqrt();
        // We must return the squared distance!
        distance * distance
    }
}

impl Universe {
    /// Create an empty universe with no systems or connections. This can be useful
    /// as a placeholder, or to extend your own universe using `ExtendedUniverse`.
    pub fn empty() -> Self {
        Self {
            systems: SystemMap(HashMap::new()),
            connections: AdjacentMap(HashMap::new()),
            rtree: rstar::RTree::new(),
        }
    }

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

    /// Extend the universe with new connections. This is useful to add additional
    /// connection, for example wormholes and find paths. The extended universe will
    /// reuse the systems from the existing universe and only take space for new connections.
    pub fn extend(&self, connections: AdjacentMap) -> ExtendedUniverse<Self> {
        ExtendedUniverse::new(self, connections)
    }
}

impl Galaxy for Universe {
    fn systems(&self) -> Vec<&System> {
        self.systems.0.values().collect::<Vec<&System>>()
    }

    fn connections(&self) -> Vec<(SystemId, SystemId)> {
        let mut connections = Vec::new();
        for adjacent in self.connections.0.values() {
            for conn in adjacent {
                connections.push((conn.from, conn.to))
            }
        }
        connections
    }
}

impl Navigatable for Universe {
    fn get_system<'a>(&self, id: &SystemId) -> Option<&System> {
        self.systems.0.get(id)
    }

    fn get_connections<'a>(&self, from: &SystemId) -> Option<Vec<Connection>> {
        self.connections.0.get(from).map(|v| v.clone())
    }

    fn get_systems_by_range<'a>(&self, from: &SystemId, range: Meters) -> Option<Vec<&System>> {
        // it is very important that we use KM, since all distances in the database are in KM, because CCP.
        let system = self.get_system(from)?;
        let systems = self
            .rtree
            .locate_within_distance(system.to_point(), range.0 * range.0)
            .filter(|s| match SecurityClass::from(s.security) {
                SecurityClass::Lowsec | SecurityClass::Nullsec => true,
                SecurityClass::Highsec => false,
            })
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
/// use neweden::source::sqlite::DatabaseBuilder;
/// use neweden::navigation::PathBuilder;
/// use neweden::Navigatable;
/// use neweden::{Connection, ConnectionType, WormholeType};
///
/// let uri = std::env::var("SQLITE_URI").unwrap();
/// let wormholes = vec![Connection {
///     from: 30002718.into(), // Rancer
///     to: 30000049.into(),  // Camal
///     type_: ConnectionType::Wormhole(WormholeType::VeryLarge),
/// }];
/// let universe = DatabaseBuilder::new(&uri).build().unwrap();
/// let extended = universe.extend(wormholes.into()); // make into an adjacent map and pass into extend()
/// let path = PathBuilder::new(&extended)
///     .waypoint(extended.get_system(&30002718.into()).unwrap()) // from Rancer
///     .waypoint(extended.get_system(&30000049.into()).unwrap()) // to Camal
///     .build() // returns an iterator
///     .collect::<Vec<_>>();
/// assert_eq!(2, path.len()); // direct jump through our wormhole
/// ```
#[derive(Debug)]
pub struct ExtendedUniverse<'a, U> {
    pub(crate) universe: &'a U,
    pub(crate) connections: AdjacentMap,
}

impl<'a, U: Galaxy + Navigatable> ExtendedUniverse<'a, U> {
    pub fn new(universe: &'a U, connections: AdjacentMap) -> Self {
        Self {
            universe,
            connections,
        }
    }
}
impl<'a, U: Galaxy> Galaxy for ExtendedUniverse<'a, U> {
    fn systems(&self) -> Vec<&System> {
        self.universe.systems()
    }

    fn connections(&self) -> Vec<(SystemId, SystemId)> {
        let mut connections = Vec::new();
        for (from, to) in self.universe.connections() {
            connections.push((from, to));
        }
        for adjacent in self.connections.0.values() {
            for conn in adjacent {
                connections.push((conn.from, conn.to))
            }
        }
        connections
    }
}

impl<'b, U: Navigatable> Navigatable for ExtendedUniverse<'b, U> {
    fn get_system<'a>(&self, id: &SystemId) -> Option<&System> {
        self.universe.get_system(id)
    }

    fn get_connections<'a>(&self, from: &SystemId) -> Option<Vec<Connection>> {
        // TODO: This is highly unoptimal
        let a = self.universe.get_connections(from);
        let b = self.connections.0.get(&from);
        match (a, b) {
            (Some(a), Some(b)) => {
                let mut v = a.clone();
                v.append(&mut b.clone());
                Some(v)
            }
            (Some(a), None) => Some(a.to_vec()),
            (None, Some(b)) => Some(b.to_vec()),
            (None, None) => None,
        }
    }

    fn get_systems_by_range<'a>(&self, from: &SystemId, range: Meters) -> Option<Vec<&System>> {
        self.universe.get_systems_by_range(from, range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_range_calculation() {
        let ly = JumpdriveShip::Titan(JumpdriveSkills::new(5, 1)).into();
        assert_eq!(Lightyears(6.0), ly);
    }
}

#[cfg(all(test, feature = "sqlite"))]
mod dbtests {
    use std::env;

    use crate::rules;
    use crate::source::sqlite::DatabaseBuilder;

    use super::*;

    extern crate test;

    #[test]
    fn test_range_query() {
        let uri = env::var("SQLITE_URI").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let camal_id = 30000049.into();
        // let faspera_id = 30000044.into();
        let systems = universe
            .get_systems_by_range(&camal_id, Lightyears(7.0).into())
            .unwrap();
        let jumpable = systems
            .into_iter()
            .filter(|x| rules::allows_cynos(x))
            .collect::<Vec<_>>();
        assert_eq!(115, jumpable.len());
    }

    #[bench]
    fn bench_range_query(b: &mut test::Bencher) {
        let uri = env::var("SQLITE_URI").expect("expected env variable DATABASE_URL set");
        let universe = DatabaseBuilder::new(&uri).build().unwrap();
        let camal_id: SystemId = 30000049.into();
        // let faspera_id = 30000044.into();
        b.iter(move || {
            test::black_box(universe.get_systems_by_range(&camal_id, Lightyears(7.0).into()));
        });
        // let jumpable = systems.into_iter().filter(|x| rules::allows_cynos(x)).collect::<Vec<_>>();
        // assert_eq!(115, jumpable.len());
    }
}
