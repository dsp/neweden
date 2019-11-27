use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemId(pub u32);
#[derive(Debug, Clone)]
pub struct SecurityStatus(pub f32);

#[derive(Debug)]
pub enum Connection {
    Jump(StargateConnection),
    Bridge(BridgeConnection),
    Wormhole(WormholeConnection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum StargateType {
    Local,
    Constellation,
    Regional,
}

#[derive(Debug)]
pub struct StargateConnection {
    pub(crate) from: SystemId,
    pub(crate) to: SystemId,
    pub(crate) jump_type: StargateType,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct BridgeConnection {}

#[derive(Debug)]
pub struct WormholeConnection {}

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
// TODO: implement PartialEq for System

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
pub struct Celestial {}

#[derive(Debug)]
pub struct Universe {
    pub(crate) systems: HashMap<SystemId, System>,
    pub(crate) connections: HashMap<SystemId, Vec<Connection>>, // adjacent map
}

impl Universe {
    pub fn get_system<'a>(&self, id: u32) -> Option<&System> {
        self.systems.get(&SystemId(id))
    }
}
