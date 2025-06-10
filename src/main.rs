// src/main.rs

mod brisk;
mod message;

use clap::Parser;
use crate::brisk::*;
use dotenv;

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

    /// Topics of the broker.
    #[arg(short, long, env = "BRISK_TOPICS", value_delimiter = ',', default_value = "brisk")]
    topics: Vec<String>,

    /// Maximum time in seconds allowed to elapse between MQTT packets sent by the client.
    #[arg(short, long, env = "BRISK_KEEP_ALIVE", default_value_t = 20)]
    keep_alive: u64,

    /// Root CA certificate for TLS connection.
    #[arg(long, env = "BRISK_ROOT_CA", default_value = None)]
    root_ca: Option<String>,

    /// Client certificate for mTLS connection.
    #[arg(long, env = "BRISK_CLIENT_CERT", default_value = None )]
    client_cert: Option<String>,

    /// Client key for mTLS connection.
    #[arg(long, env = "BRISK_CLIENT_KEY", default_value = None )]
    client_key: Option<String>,

}

fn main() {
    dotenv::from_filename(".env").ok();

    // Parse the command line arguments.
    let args: Args = Args::parse();

    let _ = Brisk::new()
        .broker(args.broker)
        .port(args.port)
        .topics(args.topics)
        .keep_alive(args.keep_alive)
        .root_ca(args.root_ca)
        .client_cert(args.client_cert)
        .client_key(args.client_key)
        .run()
        .unwrap();

}