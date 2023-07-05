# neweden: a wayfinding library for Eve Online

*neweden* is a [rust](https://rust-lang.org) library for system information,
wayfinding and range queries for the MMORPG [Eve Online](https://eveonline.com)
from CCP Games.

[![Docs](https://docs.rs/neweden/badge.svg)](https://docs.rs/neweden/)
[![Crates.io](https://img.shields.io/crates/v/neweden.svg)](https://crates.io/crates/neweden)
![License](https://img.shields.io/crates/l/neweden.svg)


## Example

### Get information about a system
```rust
use neweden::source::sqlite::DatabaseBuilder;
use neweden::Navigatable;

let universe = DatabaseBuilder::new("./sqlite-latest.sqlite").build().unwrap();
let system_id = 30000142.into(); // returns a SystemId

println!("{:?}", universe.get_system(&system_id).unwrap().name); // Jita
```

### Find a route
```rust
use neweden::source::sqlite::DatabaseBuilder;
use neweden::Navigatable;
use neweden::navigation::PathBuilder;

let universe = DatabaseBuilder::new("./sqlite-latest.sqlite").build().unwrap();
let jita = 30000142;
let camal = 30000049;
let path = PathBuilder::new(&universe)
    .waypoint(&universe.get_system(&jita.into()).unwrap())
    .waypoint(&universe.get_system(&camal.into()).unwrap())
    .build();

for system in path {
    println!("Waypoint: {}", system.name)
}

```

## Development status
The library is under development and in early alpha stages. API's will change
and your code will break. It also means that the build and test mechanism
are slightly awkward and will improve over time.

## Building
The library uses `features` to define the backends for retrieving system
and connection information. By default the library builds without any features
and you are only able to create a universe by creating universes using your own data loaders.
There are build int dataloaders for CCPs static dump. You can enable the Postgres database backend
by using the `postgres` feature or SQLite by using the `sqlite` feature.

The `rpc` feature is only for internal use and depends on a crate that is not open source.

To build the repository:
```sh
git clone https://github.com/dsp/neweden
cd neweden
cargo build --features sqlite
```

### Running tests
To run tests or benchmarks you must use the nightly. If you build with the
`database` flags you are required to provide a database connection using
the env variable `DATABASE_URL`.

```sh
git clone https://github.com/dsp/neweden
cd neweden
export SQLITE_URI="/path/to/sde/dump"
cargo +nightly test --features sqlite
```
