use anyhow;
use rusqlite;

use crate::types;

pub struct DatabaseBuilder {
    uri: String,
}

/// Loads a universe from a database.
///
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
/// println!("{:?}", universe.get_system(&system_id).unwrap().name); // Jita
/// ```
impl DatabaseBuilder {
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
        }
    }

    pub fn build(self) -> anyhow::Result<types::Universe> {
        Self::from_connection(rusqlite::Connection::open_with_flags(
            self.uri,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_URI,
        )?)
    }

    pub(self) fn from_connection(conn: rusqlite::Connection) -> anyhow::Result<types::Universe> {
        let systems = {
            let mut stm = conn.prepare(
                "
    		    SELECT solarSystemID, solarSystemName, x, y, z, security
    			FROM mapSolarSystems
    		",
            )?;

            let result = stm
                .query([])?
                .mapped(|row| {
                    Ok(types::System {
                        id: types::SystemId::from(row.get::<_, u32>(0)?),
                        name: row.get(1)?,
                        coordinate: types::Coordinate {
                            x: row.get(2)?,
                            y: row.get(3)?,
                            z: row.get(4)?,
                        },
                        security: types::Security::from(row.get::<_, f32>(5)?),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            // apparently we can't directly retrun due to borrow rules of stm
            // so we gather everything into result and return it.
            result
        };

        let connections = {
            let mut stm = conn.prepare(
                "
    		    SELECT
                    fromRegionID,
                    fromConstellationID,
                    fromSolarSystemID,
                    toSolarSystemID
                    toConstellationID,
                    toRegionID
    			FROM mapSolarSystemJumps
    		",
            )?;

            let result = stm
                .query([])?
                .mapped(|row| {
                    let from: i32 = row.get(2)?;
                    let to: i32 = row.get(3)?;
                    let stargate_type = match (
                        row.get::<_, i32>(0),
                        row.get::<_, i32>(1),
                        row.get::<_, i32>(4),
                        row.get::<_, i32>(5),
                    ) {
                        (a, _, _, b) if a != b => types::StargateType::Regional,
                        (_, a, b, _) if a != b => types::StargateType::Constellation,
                        _ => types::StargateType::Local,
                    };
                    Ok(types::Connection {
                        from: from.into(),
                        to: to.into(),
                        type_: types::ConnectionType::Stargate(stargate_type),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            result
        };

        Ok(types::Universe::new(
            types::SystemMap::from(systems),
            types::AdjacentMap::from(connections),
        ))
    }
}
