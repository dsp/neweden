#[derive(Debug)]
pub struct SystemId(pub u32);
#[derive(Debug)]
pub struct SecurityStatus(pub f32);

#[derive(Debug)]
pub enum Connection {
    Jump(StargateConnection),
    Bridge(BridgeConnection),
    Wormhole(WormholeConnection),
}

#[derive(Debug)]
pub struct StargateConnection {

}

#[derive(Debug)]
pub struct BridgeConnection {

}

#[derive(Debug)]
pub struct WormholeConnection {

}

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
pub struct Celestial {

}

#[derive(Debug)]
pub struct Universe {
    pub(crate) systems: Vec<System>,
    pub(crate) connections: Vec<Connection>,
}