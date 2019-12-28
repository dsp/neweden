# neweden: a wayfinding library for Eve Online

*neweden* is a [rust](https://rust-lang.org) library for system information,
wayfinding and range queries for the MMORPG [Eve Online](https://eveonline.com)
from CCP Games.

## Development status
The library is under development and in early alpha stages. API's will change
and your code will break. It also means that the build and test mechanism
are slightly awkward and will improve over time.

## Building
The library uses `features` to define the backends for retrieving system
and connection information. By default the library builds without any features
and you will not be able to create a universe. You can enable the Postgres
database backend by using the `database` feature. The `rpc` feature is only
for internal use and depends on a crate that is not open source.

To build the repository:
```sh
git clone https://github.com/dsp/neweden
cd neweden
cargo build --features database
```

### Running tests
To run tests or benchmarks you must use the nightly. If you build with the
`database` flags you are required to provide a database connection using
the env variable `DATABASE_URL`.

```sh
git clone https://github.com/dsp/neweden
cd neweden
cargo +nightly test --features database
```