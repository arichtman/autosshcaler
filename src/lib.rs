include!(concat!(env!("OUT_DIR"), "/_includes.rs"));

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use wol::{MacAddr, MacAddrError};

fn resolve_ip(target: IpAddr) -> Result<MacAddr, MacAddrError> {
    match target {
        IpAddr::V4(a) => match a {
            // octets is private, sadly
            // Ipv4Addr {
            //     octets: [192, 168, 1, 240],
            // } => "10:62:E5:02:09:B6".parse(),
            _ => Err(MacAddrError {}),
        },
        _ => Err(MacAddrError {}),
    }
}

// patient-zero 192.168.1.240 10:62:E5:00:B2:0D
// dr-singh 192.168.1.241 10:62:E5:00:61:A3
// smol-bat 192.168.1.242 10:62:E5:02:09:B6
