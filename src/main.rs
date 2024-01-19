#![allow(unused, irrefutable_let_patterns)]

use clap::Parser;
use domain::Targets;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, Write};

mod domain;
mod utils;
use crate::domain::CliArguments;
use crate::domain::Commands;
use crate::domain::FormatLines;
use crate::utils::{initialize_logger, mapify, write_instance, write_path, write_target};

//TODO: future enhancements
//          Add logging to all steps of the process (info/debug)
//          Ask user if they want to input another batch
//          Ask user if they want to specify a path for output
//          If file exists, ask user if they want to overwrite or add to
//          If there already exists an instance and path for one of the batches you have, add to it

fn main() -> Result<(), Error> {
    initialize_logger();
    let args = CliArguments::parse();
    let promotion_batch = match args.command {
        Commands::New(promotion_batch) => {
            promotion_batch
        },
        Commands::Add(promotion_batch) => {
            //TODO: parse current file and add to promotion batch
            promotion_batch
        },
    };
    info!("Promotion batch: {}", promotion_batch);

    // TODO: figure out a way to staticly initialize this at compile time
    let config_path = "config.toml";
    let config_file = fs::read_to_string(config_path).expect("config.toml not found");
    
    // Variable which stores the different lines we want to format arguments into (ie Instance Line)
    let format_lines: FormatLines = toml::from_str(&config_file).unwrap();

    //mutable? really?
    let mut args = mapify(promotion_batch.clone());
    let mut batches = promotion_batch.targets.clone().prepare();
    let line_type = match promotion_batch.promotion_type.as_str() {
        "image" | "images" => &format_lines.promote_images,
        "template" | "templates" => &format_lines.promote_templates,
        "secret" | "secrets" => &format_lines.promote_secrets,
        "config-map" | "config-maps" => &format_lines.promote_config_maps,
        _ => panic!("The provided promotion type is not valid {}", promotion_batch.promotion_type)
    };

    let path = "output.md";

    let mut output = fs::File::create(path)?;

    write_instance(&args, &mut output, &format_lines);
    write_path(&args, &mut output, &format_lines);
    write_target(&mut args, &mut output, &format_lines, &mut batches, line_type);
    Ok(())
}



