#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

pub mod block;
pub mod chunk;
pub mod entity;
mod utils;
pub mod world;
