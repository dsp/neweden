#![cfg_attr(test, feature(test))]

#[allow(dead_code)]
mod types;
pub use types::*;

#[cfg(feature = "database")]
#[macro_use]
extern crate diesel;
#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "database")]
#[allow(non_snake_case)]
mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
