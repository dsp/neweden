use anyhow;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::types;
use crate::schema;

type DB = diesel::pg::Pg;

pub struct DatabaseBuilder {

}

pub fn establish_connection(uri: &str) -> anyhow::Result<PgConnection> {
    let conn = PgConnection::establish(&uri)?;
    Ok(conn)
}

impl DatabaseBuilder {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn finish(&self) -> types::Universe {
        unimplemented!();
    }
}

impl Queryable<schema::mapSolarSystems::SqlType, DB> for types::System {
    type Row = (
        Option<i32>,
        i32,
        Option<String>,
        Option<f64>,
        Option<f64>,
        Option<f64>,
        Option<f64>,
        Option<f64>,
        Option<String>,
    );

    fn build(row: Self::Row) -> Self {
        types::System {
            id: types::SystemId(row.1 as u32),
            name: row.2.unwrap(),
            coordinate: types::Coordinate {
                x: row.3.unwrap() as f32,
                y: row.4.unwrap() as f32,
                z: row.5.unwrap() as f32,
            },
            security: types::SecurityStatus(row.7.unwrap() as f32),
        }
    }
}

impl Queryable<schema::mapSolarSystemJumps::SqlType, DB> for types::Connection {
    type Row = (
        Option<i32>, // fromRegionID
        Option<i32>, // fromConstellationID,
        i32,         // fromSolarSystemID
        i32,         // toSolarSystemID
        Option<i32>, // toConstellationID,
        Option<i32>, // toRegionID,
    );

    fn build(row: Self::Row) -> Self {
        types::Connection::Jump(
            types::StargateConnection {
                from: types::SystemId(row.2 as u32),
                to: types::SystemId(row.3 as u32),
                jump_type: match (row.0, row.1, row.4, row.5) {
                    (a, _, _, b) if a != b => types::StargateType::Regional,
                    (_, a, b, _) if a != b => types::StargateType::Constellation,
                    _ => types::StargateType::Local,
                },
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use schema::mapSolarSystems::dsl::*;
    use schema::mapSolarSystemJumps::dsl::*;

    #[test]
    fn test_simple_system_query() {
        let uri = env::var("DATABASE_URL")
            .expect("expected env variable DATABASE_URL set");
        let conn = establish_connection(&uri)
            .expect("expected postgres connection to be established");
        let system = mapSolarSystems
            .filter(solarSystemID.eq(30000049))
            .limit(1)
            .load::<types::System>(&conn)
            .expect("first row to be returned from postgres");
        assert_eq!("Camal", system[0].name);
    }

    #[test]
    fn test_simple_connection_query() {
        let uri = env::var("DATABASE_URL")
            .expect("expected env variable DATABASE_URL set");
        let conn = establish_connection(&uri)
            .expect("expected postgres connection to be established");
        let res = mapSolarSystemJumps
            .filter(
                fromSolarSystemID.eq(30000049).and(toSolarSystemID.eq(30000045))
                .or(
                    fromSolarSystemID.eq(30000015).and(toSolarSystemID.eq(30001047))))
            .limit(2)
            .order_by(fromSolarSystemID)
            .load::<types::Connection>(&conn)
            .expect("expect connection");
        match (&res[0], &res[1]) {
            (types::Connection::Jump(sg1), types::Connection::Jump(sg2)) => {
                assert_eq!(sg1.from, types::SystemId(30000015));
                assert_eq!(sg1.to, types::SystemId(30001047));
                assert_eq!(sg2.from, types::SystemId(30000049));
                assert_eq!(sg2.to, types::SystemId(30000045));
                assert_eq!(sg1.jump_type, types::StargateType::Regional);
                assert_eq!(sg2.jump_type, types::StargateType::Local);
            },
            _ => assert!(false), 
        }
    }
}