use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SystemId(pub u32);
#[derive(Debug)]
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
pub struct BridgeConnection {}

#[derive(Debug)]
pub struct WormholeConnection {}

#[derive(Debug)]
pub struct Coordinate {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Debug)]
pub struct System {
    pub(crate) id: SystemId,
    pub(crate) name: String,
    pub(crate) coordinate: Coordinate,
    pub(crate) security: SecurityStatus,
}
// TODO: implement PartialEq for System

#[derive(Debug)]
pub struct Celestial {}

#[derive(Debug)]
pub struct Universe {
    pub(crate) systems: HashMap<SystemId, System>,
    pub(crate) connections: Vec<Connection>,
}
