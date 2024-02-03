use std::collections::HashMap;
use strfmt::strfmt;

use crate::domain::PromotionBatch;
use crate::domain::Targets;

pub fn mapify(batch: PromotionBatch) -> HashMap<String, String> {
    let mut args: HashMap<String, String> = HashMap::new();
    args.insert("instance".to_string(), batch.instance);
    args.insert("source".to_string(), batch.source);
    args.insert("destination".to_string(), batch.destination);
    args
}

pub fn print_instance(args: &HashMap<String, String>, line_type: &str) {
    println!("{}", strfmt(line_type, args).unwrap());
}

pub fn print_path(args: &HashMap<String, String>, line_type: &str) {
    println!("{}", strfmt(line_type, args).unwrap());
}

pub fn print_target(args: &mut HashMap<String, String>, batches: &mut Vec<Targets>, line_type: &str) {
    #[allow(irrefutable_let_patterns)]
    while let targets = batches.remove(0) {
        args.insert("targets".to_string(), targets.to_string());
        println!("{}", strfmt(line_type, &args).unwrap());
        if batches.is_empty() {
            break;
        }
    }
}
