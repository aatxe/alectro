// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate futures;
extern crate irc;
extern crate termion;
extern crate tokio_core;
extern crate unicode_segmentation;
extern crate unicode_width;


#[macro_use]
extern crate error_chain;

pub mod controller;
mod error;
pub mod model;
pub mod view;
