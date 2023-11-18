#![allow(unused, dead_code)]

use read_input::prelude::*;

use log::{info, warn, debug};
use clap::{Parser};

mod domain;
mod utils;
use crate::domain::Targets;
use crate::domain::CliArguments;
use crate::domain::OneLineArguments;
use crate::domain::Commands;
use crate::utils::initialize_logger;


fn test_command() {
    println!("Tested this command MFER!")
}

//TODO:
// begin commenting structs to describe their purpose
fn main() {
    initialize_logger();
    let args = CliArguments::parse();
    match &args.command {
        Commands::Prompt => {
            println!("ooblah")
        },
        Commands::Line(one) => {
            debug!("Arguments provided: {:?}", one);
            println!("Nice targets bro!! {}", one.targets);
            print!("read input test: ");
            let targets = input::<Targets>().get();
            println!("Nice input bro: {}", targets);
        }
    }
}
