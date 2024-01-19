use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use strfmt::strfmt;

use read_input::prelude::*;

use env_logger::Env;

use crate::domain::FormatLines;
use crate::domain::PromotionBatch;
use crate::domain::Targets;

pub fn initialize_logger() {
    let env = Env::new()
        .filter_or("MY_LOG", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env)
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
    } else if promotion_type.eq_ignore_ascii_case("config-maps") 
    || promotion_type.eq_ignore_ascii_case("config-map") 
    || promotion_type.eq_ignore_ascii_case("templates") 
    || promotion_type.eq_ignore_ascii_case("template") {
        for target in targets.iter() {
            if target.contains(':') {
                err_list.push(target.clone());
            }
        }
    }
    err_list
}

pub fn read_file(file_name: &str, format_lines: &FormatLines) -> Vec<PromotionBatch> {
    // 1. Open file with BufReader
    // 2. Convert to Vec of lines
    // 3. Parse instance line
    // 4. Parse path line (should be 2 down from instance line)
    // 5. Parse all promote lines (1 of 4)
    // 6. Push promotion batch to list
    // 7. Find next header line (path or instance)
    // 8. Create new promotion batch, start again from 5
    let path = "output.md";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let input = reader.lines()
        .flatten()
        .collect::<Vec<String>>();

    todo!()
}

pub fn write_instance(args: &HashMap<String, String>, output: &mut File, format_lines: &FormatLines) {
    // write instance + instance subline
    writeln!(output, "{}", strfmt(&format_lines.instance, args).unwrap()).unwrap();
    writeln!(output, "{}", strfmt(&format_lines.instance_subline, args).unwrap()).unwrap();
}

pub fn write_path(args: &HashMap<String, String>, output: &mut File, format_lines: &FormatLines) {
    // write path
    writeln!(output, "{}", strfmt(&format_lines.path, &args).unwrap()).unwrap();
}

pub fn write_target(args: &mut HashMap<String, String>, output: &mut File,
    format_lines: &FormatLines, batches: &mut Vec<Targets>, line_type: &str) {
    // write target bullets
    while let targets = batches.remove(0) {
        args.insert("targets".to_string(), targets.to_string());
        writeln!(output, "{}", strfmt(line_type, &args).unwrap()).unwrap();
        if batches.is_empty() {
            break;
        }
    }
}
