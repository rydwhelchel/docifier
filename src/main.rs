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
use crate::utils::parse_prompt;

//TODO: begin commenting structs to describe their purpose
fn main() {
    initialize_logger();
    let args = CliArguments::parse();
    let promotion_batch = match args.command {
        Commands::Prompt => {
            let promotion_batch = parse_prompt();
            promotion_batch
        },
        Commands::Line(promotion_batch) => {
            debug!("Arguments provided: {:?}", promotion_batch);
            promotion_batch
        }
    };
    println!("Nice batch bro!! {}", promotion_batch);
}
