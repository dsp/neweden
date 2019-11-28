use anyhow;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema;
use crate::schema::mapSolarSystemJumps::dsl::*;
use crate::schema::mapSolarSystems::dsl::*;
use crate::types;

type DB = diesel::pg::Pg;

pub struct DatabaseBuilder {
    uri: String,
}

pub fn establish_connection(uri: &str) -> anyhow::Result<PgConnection> {
    let conn = PgConnection::establish(&uri)?;
    Ok(conn)
}

impl DatabaseBuilder {
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
        }
    }

    pub fn build(&self) -> anyhow::Result<types::Universe> {
        let conn = PgConnection::establish(&self.uri)?;
        Self::from_connection(&conn)
    }

    pub(self) fn from_connection(conn: &PgConnection) -> anyhow::Result<types::Universe> {
        let systems = mapSolarSystems
            // this is k-space and w-space
            .filter(solarSystemID.lt(32000000))
            .load::<types::System>(conn)?;

        let jumps = mapSolarSystemJumps
            .filter(
                // only query k-space since w-space has no connections
                fromSolarSystemID
                    .lt(31000000)
                    .and(toSolarSystemID.lt(31000000)),
            )
            .load::<types::Connection>(conn)?;

        Ok(types::Universe::new(systems.into(),jumps.into()))
    }
}

impl Queryable<schema::mapSolarSystems::SqlType, DB> for types::System {
    type Row = (
        Option<i32>,    // regionID
        i32,            // solarSystemID
        Option<String>, // solarSystemName
        Option<f64>,    // x
        Option<f64>,    // y
        Option<f64>,    // z
        Option<f64>,    // luminosity
        Option<f64>,    // security
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
            security: types::Security(row.7.unwrap() as f32),
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
        let stargate_type = match (row.0, row.1, row.4, row.5) {
            (a, _, _, b) if a != b => types::StargateType::Regional,
            (_, a, b, _) if a != b => types::StargateType::Constellation,
            _ => types::StargateType::Local,
        };

        types::Connection {
            from: types::SystemId(row.2 as u32),
            to: types::SystemId(row.3 as u32),
            type_: types::ConnectionType::Stargate(stargate_type),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[test]
    fn test_simple_system_query() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let conn =
            establish_connection(&uri).expect("expected postgres connection to be established");
        let system = mapSolarSystems
            .filter(solarSystemID.eq(30000049))
            .limit(1)
            .load::<types::System>(&conn)
            .expect("first row to be returned from postgres");
        assert_eq!("Camal", system[0].name);
    }

    #[test]
    fn test_simple_connection_query() {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let conn =
            establish_connection(&uri).expect("expected postgres connection to be established");
        let res = mapSolarSystemJumps
            .filter(
                fromSolarSystemID
                    .eq(30000049)
                    .and(toSolarSystemID.eq(30000045))
                    .or(fromSolarSystemID
                        .eq(30000015)
                        .and(toSolarSystemID.eq(30001047))),
            )
            .limit(2)
            .order_by(fromSolarSystemID)
            .load::<types::Connection>(&conn)
            .expect("expect connection");
        let (sg1, sg2) = (&res[0], &res[1]);
        assert_eq!(sg1.from, types::SystemId(30000015));
        assert_eq!(sg1.to, types::SystemId(30001047));
        assert_eq!(sg2.from, types::SystemId(30000049));
        assert_eq!(sg2.to, types::SystemId(30000045));
        if let types::ConnectionType::Stargate(jt) = &sg1.type_ {
            assert_eq!(jt, &types::StargateType::Regional);
        }
        if let types::ConnectionType::Stargate(jt) = &sg2.type_ {
            assert_eq!(jt, &types::StargateType::Local);
        }
    }
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use std::env;

    #[bench]
    fn bench_simple_system_query(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let conn = PgConnection::establish(&uri).expect("establish connection");
        b.iter(|| {
            test::black_box(
                mapSolarSystems
                    .filter(solarSystemID.eq(30000049))
                    .limit(1)
                    .load::<types::System>(&conn)
                    .expect("first row to be returned from postgres"),
            );
        });
    }

    #[bench]
    fn bench_build_universe(b: &mut test::Bencher) {
        let uri = env::var("DATABASE_URL").expect("expected env variable DATABASE_URL set");
        let conn = PgConnection::establish(&uri).expect("establish connection");
        b.iter(|| {
            test::black_box(DatabaseBuilder::from_connection(&conn).unwrap());
        });
    }
}
