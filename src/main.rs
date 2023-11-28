// TODO: remove for release
#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use log::{debug, error, info, trace, warn};
use std::net::IpAddr;

use autosshcale_rs;
use clap::Parser;
use wol::MacAddr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    #[clap(short, long, value_parser, env)]
    ip_address: IpAddr,
    #[clap(
        short,
        long,
        value_parser,
        env,
        long_help = "18-character MAC address as string. Delimiter inferred."
    )]
    mac_address: String,
    /// Increments logging verbosity.
    #[arg(short, long, action = clap::ArgAction::Count, env, long_help = "Optional. May be applied up to 4 times. Environment variable takes integer.")]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    let log_level = match args.verbose {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        3 => log::Level::Debug,
        4.. => log::Level::Trace,
    };
    simple_logger::init_with_level(log_level).expect("Error initialising logging, aborting.");
    info!("Log level {}", args.verbose);
    debug!("{args:?}");
    let ip_address = args.ip_address;
    let mac_address = args.mac_address;
}
