extern crate uuid;

#[macro_use]
mod entity_manager;

pub use entity_manager::*;

#[cfg(test)]
mod test;