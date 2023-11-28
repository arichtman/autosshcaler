use std::net::IpAddr;

use autosshcale_rs;
use clap::Parser;
use wol::MacAddr;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(name = "IP_ADDRESS", short = 'a', long = "ip-address", value_parser)]
    ip_address: IpAddr,
}

fn main() {
    let args = Args::parse();
    let ip_address: IpAddr = args.ip_address;
    println!("Got {ip_address}");
}
