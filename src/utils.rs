use std::collections::HashMap;

use read_input::prelude::*;

use env_logger::Env;

use crate::domain::PromotionBatch;
use crate::domain::Targets;

pub fn initialize_logger() {
    let env = Env::new()
        .filter_or("MY_LOG", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
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

    print!(
        "Please provide a comma separated list of the {}: ",
        promotion_type
    );
    let mut targets = input::<Targets>().get();

    //TODO: this whole section feels ugly, review later to see if there is a better way to do this
    //      currently this just validates images, should have a branching path here to validate templates/config maps/secrets (no colon)
    //      maybe move this section into another method to parse the targets ? idk
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

    //TODO: Consider adding a y/n prompt asking if the user wants to insert an additional promotion batch
    //      that way we can actually create a list of batches to insert into the file at once
    //      if we do that, we need to change the return type of this method to a vec of promotionbatches
    //TODO: The above logic would need to be in the location which this function is called in

    PromotionBatch {
        instance,
        source,
        destination,
        promotion_type,
        targets,
    }
}

pub fn mapify(batch: PromotionBatch) -> HashMap<String, String> {
    let mut args: HashMap<String, String> = HashMap::new();
    args.insert("instance".to_string(), batch.instance);
    args.insert("source".to_string(), batch.source);
    args.insert("destination".to_string(), batch.destination);
    args.insert("promotion_type".to_string(), batch.promotion_type);
    args
}

pub fn validate_targets(promotion_type: &str, targets: Targets) -> Vec<String> {
    let mut err_list: Vec<String> = vec![];
    if promotion_type.eq_ignore_ascii_case("images") || promotion_type.eq_ignore_ascii_case("image") {
        for target in targets.iter() {
            if !target.contains(':') {
                err_list.push(target.clone());
            }
        }
        err_list
    } else {
        err_list
    }
}
