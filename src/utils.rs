use env_logger::{Logger,Env};

pub fn initialize_logger() -> () {
    let env = Env::new()
        .filter_or("MY_LOG", "debug")    // filters out any messages that aren't at "debug" log level or above
        .write_style_or("MY_LOG_STYLE", "always");    // always use styles when printing
    env_logger::init_from_env(env)
}