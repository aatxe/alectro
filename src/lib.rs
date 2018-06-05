// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;
extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;
extern crate unicode_segmentation;
extern crate unicode_width;

pub mod controller;
mod error;
pub mod input;
pub mod model;
mod utils;
pub mod view;
