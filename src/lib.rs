#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

#[macro_use]
extern crate lazy_static;
pub use lazy_static::lazy_static;

pub mod block;
pub mod chunk;
pub mod events;
pub mod utils;
pub mod world;
