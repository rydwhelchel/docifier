#![allow(unused, irrefutable_let_patterns)]

use clap::Parser;
use log::{debug, info, warn};
use std::fs;
use std::io::{Error, Write};
use strfmt::strfmt;

mod domain;
mod utils;
use crate::domain::CliArguments;
use crate::domain::Commands;
use crate::domain::FormatLines;
use crate::utils::{initialize_logger, mapify};
use crate::utils::parse_prompt;

//TODO: future enhancements
//          Add logging to all steps of the process (info/debug)
//          Ask user if they want to input another batch
//          Ask user if they want to specify a path for output
//          If file exists, ask user if they want to overwrite or add to
//          If there already exists an instance and path for one of the batches you have, add to it

//TODO: Potential flow of adding to a file
//      Loop through lines, if you find a instance+path, loop through remaining_batches
//          if you find a match, add it to that section and then remove it from remaining_batches
//      continue this process until you hit the end of the file or you hit the end of remaining_batches
//      if eof is hit and there are more in remaining batches, process remaining_batches to group up by instance and path and then
//          write to file

fn main() -> Result<(), Error> {
    initialize_logger();
    let args = CliArguments::parse();
    let promotion_batch = match args.command {
        Commands::Prompt => parse_prompt(),
        Commands::Line(promotion_batch) => {
            debug!("Arguments provided: {:?}", promotion_batch);
            //TODO: Need to enforce that promotion_type is an accepted type
            promotion_batch
        }
    };
    println!("Nice batch bro!! {}", promotion_batch);

    let config_path = "config.toml";
    let config_file = fs::read_to_string(config_path).expect("config.toml not found");
    let format_lines: FormatLines = toml::from_str(&config_file).unwrap();
    let mut args = mapify(promotion_batch.clone());
    let mut batches = promotion_batch.targets.clone().prepare();

    let path = "output.md";

    let mut output = fs::File::create(path)?;
    writeln!(output, "{}", strfmt(&format_lines.instance, &args).unwrap())?;
    writeln!(output, "{}", strfmt(&format_lines.instance_subline, &args).unwrap())?;
    writeln!(output, "{}", strfmt(&format_lines.path, &args).unwrap())?;
    while let targets = batches.remove(0) {
        args.insert("targets".to_string(), targets.to_string());
        writeln!(output, "{}", strfmt(&format_lines.promote_images, &args).unwrap())?;
        if batches.is_empty() {
            break;
        }
    }
    Ok(())
}
