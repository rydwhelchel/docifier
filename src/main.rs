#![allow(unused, dead_code)]

use read_input::prelude::*;

use log::{info, warn, debug};
use clap::{Parser};

mod domain;
mod utils;
use crate::domain::Targets;
use crate::domain::CliArguments;
use crate::domain::PromotionBatch;
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
            print!("Please provide the instance for the source and destination: ");
            let instance = input::<String>().get();
            print!("Please provide the source env: ");
            let source = input::<String>().get();
            print!("Please provide the destination env: ");
            let destination = input::<String>().get();
            print!("Please provide the promotion type: ");
            let promotion_type = input::<String>().get();

            //TODO: Have some branching paths here on what user input should look like
            //      IE: if images, should probably have a colon in the name of each image to denote version
            //      throw warning and ask user if they intended to leave out a colon on each entry
            print!("Please provide a comma separated list of the {}: ", promotion_type);
            let targets = input::<Targets>().get();

            let promotion_batch: PromotionBatch = PromotionBatch {
                instance,
                source,
                destination,
                promotion_type,
                targets
            };
            println!("Nice args bro: {}", promotion_batch);
        },
        Commands::Line(one) => {
            debug!("Arguments provided: {:?}", one);
            println!("Nice batch bro!! {}", one);
        }
    }
}
