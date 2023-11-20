#![allow(unused, dead_code)]

use read_input::prelude::*;

use clap::Parser;
use log::{debug, info, warn};

mod domain;
mod utils;
use crate::domain::CliArguments;
use crate::domain::Commands;
use crate::domain::PromotionBatch;
use crate::domain::Targets;
use crate::utils::initialize_logger;
use crate::utils::parse_prompt;

//TODO: begin commenting structs to describe their purpose
fn main() {
    initialize_logger();
    let args = CliArguments::parse();
    let promotion_batch = match args.command {
        Commands::Prompt => parse_prompt(),
        Commands::Line(promotion_batch) => {
            debug!("Arguments provided: {:?}", promotion_batch);
            promotion_batch
        }
    };
    println!("Nice batch bro!! {}", promotion_batch);
}
