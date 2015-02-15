#![feature(plugin, io, path, collections, std_misc, core, env)]
#![plugin(regex_macros, docopt_macros)]
extern crate regex;
#[no_link] extern crate regex_macros;
extern crate docopt;
#[no_link] extern crate docopt_macros;
#[macro_use] extern crate log;
extern crate term;
extern crate toml;
extern crate time;
extern crate uuid;
extern crate "rustc-serialize" as rustc_serialize;

pub mod errors;
pub mod git;
pub mod utils;
pub mod config;
pub mod job;
