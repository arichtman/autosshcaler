use autosshcale_rs;
use clap::Parser;
use wol::MacAddr;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(name = "MAC_ADDRESS", short = 'm', long = "mac-address", value_parser)]
    mac_address: String,
}

fn main() {
    let args = Args::parse();
    let mac_address: MacAddr = args.mac_address.parse().unwrap();
    println!("Got {mac_address}");
}
