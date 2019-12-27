/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */

#![cfg_attr(test, feature(test))]

#[cfg(feature = "database")]
#[macro_use]
extern crate diesel;
#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "database")]
#[allow(non_snake_case)]
mod schema;

pub use types::*;
#[allow(dead_code)]
pub mod rules;
#[allow(dead_code)]
mod types;

#[allow(dead_code)]
pub mod navigation;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
