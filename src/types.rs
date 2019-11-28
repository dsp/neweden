use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityStatus(pub f32);

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityClass {
    Highsec,
    Lowsec,
    Nullsec
}

impl From<SecurityStatus> for SecurityClass {
    fn from(other: SecurityStatus) -> Self {
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

#[derive(Debug)]
pub struct Connection {
    pub(crate) from: SystemId,
    pub(crate) to: SystemId,
    pub(crate) type_: ConnectionType,
}

#[derive(Debug)]
pub enum ConnectionType {
    Stargate(StargateType),
    Bridge(BridgeType),
    Wormhole(WormholeType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeType {
    // TODO: introduce a type JumpDrive
    Titan(u8, u8), // jump drive calibration, jump fuel conservation
    BlackOps(u8, u8), // jump drive calibration, jump fuel conservation
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StargateType {
    Local,
    Constellation,
    Regional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WormholeType {
    VeryLarge, // everything, except supers+
    Large, // battleships
    Medium, // battlecruisers, etc
    Small, // frigates, etc
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SystemClass {
    KSpace,
    WSpace,
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

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Debug, Clone)]
pub struct System {
    pub(crate) id: SystemId,
    pub(crate) name: String,
    pub(crate) coordinate: Coordinate,
    pub(crate) security: SecurityStatus,
}

impl System {
    pub fn get_id(&self) -> u32 {
        self.id.0
    }
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

pub trait Navigatable {
    fn get_system<'a>(&self, id: SystemId) -> Option<&System>;
    fn get_connections<'a>(&self, from: SystemId) -> Option<&Vec<Connection>>;
}

#[derive(Debug)]
pub struct Universe {
    pub(crate) systems: SystemMap,
    pub(crate) connections: AdjacentMap,
}

impl Navigatable for Universe {
    fn get_system<'a>(&self, id: SystemId) -> Option<&System> {
        self.systems.0.get(&id)
    }

    fn get_connections<'a>(&self, from: SystemId) -> Option<&Vec<Connection>> {
        self.connections.0.get(&from)
    }
}

#[derive(Debug)]
pub struct ExtendedUniverse<'a> {
    universe: &'a Universe,
    connections: AdjacentMap,
}

impl<'a> ExtendedUniverse<'a> {
    pub fn new(universe: &'a Universe, connections: AdjacentMap) -> Self {
        Self {
            universe: universe,
            connections: connections,
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
}
