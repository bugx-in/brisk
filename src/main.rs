// src/main.rs

mod brisk;
mod message;

use clap::Parser;
use crate::brisk::*;

/// Brisk command line interface.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hostname of the broker.
    #[arg(short, long, env = "BRISK_BROKER")]
    broker: String,

    /// Port of the broker.
    #[arg(short, long, env = "BRISK_BROKER_PORT", default_value_t = 1883)]
    port: u16,

    /// Topic of the broker.
    #[arg(short, long, env = "BRISK_TOPICS", value_delimiter = ',')]
    topics: Vec<String>,

    /// Topic of the broker.
    #[arg(short, long, env = "BRISK_KEEP_ALIVE", default_value_t = 5)]
    keep_alive: u64,

}

fn main() {
    // Parse the command line arguments.
    let args: Args = Args::parse();

    let _ = Brisk::new()
        .broker(args.broker)
        .port(args.port)
        .topics(args.topics)
        .keep_alive(args.keep_alive)
        .run()
        .unwrap();

}