#![allow(unused, dead_code)]

use log::{info, warn, debug};
use clap::{Parser};

mod domain;
mod utils;
use crate::domain::Subjects;
use crate::domain::CliArguments;
use crate::utils::initialize_logger;


//TODO:
// begin commenting structs to describe their purpose
fn main() {
    initialize_logger();
    let args = CliArguments::parse();
    debug!("Arguments provided: {:?}", args);
    println!("Nice subjects bro!! {}", args.subjects)
}
