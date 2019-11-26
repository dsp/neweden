#[macro_use]
extern crate diesel;

#[allow(dead_code)]
mod types;
pub use types::*;

#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "database")]
#[allow(non_snake_case)]
mod schema;

pub trait Navigatable {
    fn get_connections(&self, system: &types::System) -> Vec<types::Connection>;
    fn get_systems(&self) -> Vec<types::System>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
