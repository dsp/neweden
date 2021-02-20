/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */
#![cfg_attr(test, feature(test))]
//! neweden is a rust library for system information, wayfinding and
//! range queries for the MMORPG Eve Online from CCP Games.
//!
//! Online data can come from multiple data sources. Most commonly
//! a CCP static dump from https://www.fuzzwork.co.uk/dump/.
//!
//! The library must be compiled with the apprioriate flags. Currently
//! accepted flags are `database` and `rpc`. `database` offers a Postgres
//! backend using the diesel ORM wrapper. `rpc` is for internal use at
//! the moment as the dependent crate is not open sourced.

// Must be at the crate root
#[cfg(feature = "database")]
#[macro_use]
extern crate diesel;

pub mod source;

#[allow(dead_code)]
mod builder;
pub use builder::*;

#[allow(dead_code)]
pub mod rules;
#[allow(dead_code)]
mod types;
pub use types::*;

#[allow(dead_code)]
pub mod navigation;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
