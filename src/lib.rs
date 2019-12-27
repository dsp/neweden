/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */

#![cfg_attr(test, feature(test))]

// Must be at the crate root
#[cfg(feature = "database")]
#[macro_use]
extern crate diesel;

pub mod source;

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
