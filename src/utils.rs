use read_input::prelude::*;

use env_logger::{Env, Logger};

use crate::domain::PromotionBatch;
use crate::domain::Targets;

pub fn initialize_logger() {
    let env = Env::new()
        .filter_or("MY_LOG", "debug") // filters out any messages that aren't at "debug" log level or above
        .write_style_or("MY_LOG_STYLE", "always"); // always use styles when printing
    env_logger::init_from_env(env)
}

/// Prompts the user to input all relevant fields for a PromotionBatch
///     then collects user's input and constructs and returns a PromotionBatch
pub fn parse_prompt() -> PromotionBatch {
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
    print!(
        "Please provide a comma separated list of the {}: ",
        promotion_type
    );
    let mut targets = input::<Targets>().get();

    //TODO: this whole section feels ugly, review later to see if there is a better way to do this
    loop {
        //beware this targets.clone(), could be expensive, especially since its in a loop
        let err_list = validate_targets(&promotion_type, targets.clone());
        if !err_list.is_empty() {
            println!("The following targets do not match our schema for the provided promotion_type : {:?}", err_list);
            println!("If you would like to update, please provide a new comma separated list with the updated targets list");
            print!("Otherwise, just press enter and we'll continue on with the existing targets list: ");
            let new_targets = input::<Targets>().get();
            if new_targets.len() > 0 {
                targets = new_targets;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    PromotionBatch {
        instance,
        source,
        destination,
        promotion_type,
        targets,
    }
}

pub fn validate_targets(promotion_type: &str, targets: Targets) -> Vec<String> {
    let mut err_list: Vec<String> = vec![];
    if promotion_type.eq_ignore_ascii_case("images") {
        for target in targets.iter() {
            if !target.contains(':') {
                //Probably a horrible way of doing this, way too much cloning
                err_list.push(target.clone());
            }
        }
        err_list
    } else {
        println!("Ur mom");
        err_list
    }
}
