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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use schema::mapSolarSystems::dsl::*;

    #[test]
    fn test_me() {
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
}