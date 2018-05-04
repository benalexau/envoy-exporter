extern crate hyper;
extern crate curl;
extern crate serde_json;
extern crate toml;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate serde_derive;

mod config;
mod exporter;
mod envoy_reader;

use std::env;
use config::Config;
use exporter::Exporter;

static BUILD_TIME: Option<&'static str> = option_env!("BUILD_TIME");
static GIT_REVISION: Option<&'static str> = option_env!("GIT_REVISION");
static RUST_VERSION: Option<&'static str> = option_env!("RUST_VERSION");
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let version_info = if BUILD_TIME.is_some() {
        format!(
            "  version   : {}\n  revision  : {}\n  build time: {}\n",
            VERSION,
            GIT_REVISION.unwrap_or(""),
            BUILD_TIME.unwrap()
        )
    } else {
        format!("  version: {}\n", VERSION)
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: envoy-exporter [config_file]");
        println!("\n{}", version_info);
        return;
    }

    let config = match Config::from_file(&args[1]) {
        Ok(x) => x,
        Err(x) => {
            println!("Could not read '{}': {}", &args[1], x);
            return;
        }
    };

    match Exporter::start(config) {
        Ok(_) => (),
        Err(x) => {
            println!("Server failed: {}", x);
            return;
        }
    }
}
