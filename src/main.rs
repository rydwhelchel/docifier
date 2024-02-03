use clap::Parser;
use static_toml::static_toml;
use std::io::Error;

mod domain;
mod utils;
use crate::domain::PromotionBatch;
use crate::utils::{mapify, print_instance, print_path, print_target};

fn main() -> Result<(), Error> {
    let batch = PromotionBatch::parse();

    static_toml! {
        static FORMAT_LINES = include_toml!("config.toml");
    }

    let mut args = mapify(batch.clone());
    let mut batches = batch.targets.clone().prepare();
    let line_type = match batch.promotion_type.as_str() {
        "image" | "images" => &FORMAT_LINES.promote_images,
        "secret" | "secrets" => &FORMAT_LINES.promote_secrets,
        "config-map" | "config-maps" | "config_map" | "config_maps" => &FORMAT_LINES.promote_config_maps,
        _ => panic!("The provided promotion type is not valid {}", batch.promotion_type)
    };

    //echo output
    print_instance(&args, &FORMAT_LINES.instance);
    print_path(&args, &FORMAT_LINES.path);
    print_target(&mut args, &mut batches, line_type);

    Ok(())
}



