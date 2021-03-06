//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// #![feature(plugin, path_ext, convert)]
extern crate regex;
#[macro_use] extern crate log;
extern crate term;
extern crate toml;
extern crate time;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate rpassword;
extern crate libc;
extern crate tempdir;
extern crate uuid;
#[macro_use] extern crate hyper;
extern crate mime;
extern crate clap;
extern crate crypto;
#[cfg(test)] extern crate mockito;

#[macro_export]
macro_rules! validate {
    ($config:ident, $value:ident) => (
        try!($config.$value());
    )
}

// Adding a quick macro to assert enums, specifically to test
// the error::Kind enum since we can't add #[derive(PartialEq)]
// because hyper and other types doesn't implement it
#[cfg(test)]
#[macro_export]
macro_rules! assert_enum {
    ($enum1:expr, $enum2:pat) => (
        match $enum1 {
            $enum2 => true,
            _ => false
        }
    )
}

pub mod errors;
pub mod types;
pub mod git;
pub mod utils;
pub mod config;
pub mod delivery_config;
pub mod job;
pub mod getpass;
pub mod token;
pub mod http;
pub mod project;
pub mod cookbook;
pub mod cli;
pub mod command;
pub mod fips;
pub mod json;
