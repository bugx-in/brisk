// src/main.rs

mod brisk;
mod message;

use clap::Parser;
use crate::brisk::*;
use log::LevelFilter;
use env_logger;
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
    #[arg(long, env = "BRISK_CLIENT_CERT", default_value = None, requires = "client_key" )]
    client_cert: Option<String>,

    /// Client key for mTLS connection.
    #[arg(long, env = "BRISK_CLIENT_KEY", default_value = None, requires = "client_cert" )]
    client_key: Option<String>,

    /// Log level
    #[arg(short, long, env = "BRISK_LOG_LEVEL", default_value = "info")]
    log_level: String,

    /// Username
    #[arg(short = 'P', long, env = "BRISK_USERNAME", default_value = None, requires = "password")]
    username: Option<String>,

    /// Password
    #[arg(short = 'U', long, env = "BRISK_PASSWORD", default_value = None, hide_env_values= true, requires = "username")]
    password: Option<String>,
}

fn main() {
    dotenv::from_filename(".env").ok();

    // Parse the command line arguments.
    let args: Args = Args::parse();

    // Initialize logger.
    let log_level = match args.log_level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    };
    env_logger::Builder::new()
        .filter_level(log_level)
        .init();

    // Run brisk.
    let _ = Brisk::new()
        .broker(&args.broker)
        .port(&args.port)
        .topics(&args.topics)
        .keep_alive(&args.keep_alive)
        .root_ca(&args.root_ca)
        .client_cert(&args.client_cert)
        .client_key(&args.client_key)
        .username(&args.username)
        .password(&args.password)
        .run()
        .unwrap();

}