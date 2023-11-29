// TODO: remove for release
#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use anyhow::Result;
use log::{debug, error, info, trace, warn};
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use std::{thread, time};

use autosshcale_rs;
use clap::Parser;
use russh::*;
use russh_keys::*;
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;
use wol::{send_wol, MacAddr};

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

#[tokio::main]
async fn main() -> Result<()> {
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
    let mac_address: MacAddr = args.mac_address.parse().unwrap();
    send_wol(mac_address, None, None)?;
    let thirty_seconds = time::Duration::from_secs(30);
    thread::sleep(thirty_seconds);
    let private_key: &Path = Path::new("./id_ed25519");
    info!("Commencing SSH connection to {ip_address} as nixos");
    let mut ssh = Session::connect(private_key, "nixos".to_string(), (ip_address, 22))?;
}

/// This struct is a convenience wrapper
/// around a russh client
pub struct Session<Client: russh::client::Handler> {
    session: client::Handle<Client>,
}

impl Session<Client> {
    async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: P,
        user: impl Into<String>,
        addrs: A,
    ) -> Result<Self> {
        let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;
        let auth_res = session
            .authenticate_publickey(user, Arc::new(key_pair))
            .await?;

        if !auth_res {
            anyhow::bail!("Authentication failed");
        }

        Ok(Self { session })
    }

    async fn call(&mut self, command: &str) -> Result<u32> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut code = 0;
        let mut stdout = tokio::io::stdout();

        loop {
            // There's an event available on the session channel
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                // Write data to the terminal
                ChannelMsg::Data { ref data } => {
                    stdout.write_all(data).await?;
                    stdout.flush().await?;
                }
                // The command has returned an exit code
                ChannelMsg::ExitStatus { exit_status } => {
                    code = exit_status;
                    channel.eof().await?;
                    break;
                }
                _ => {}
            }
        }
        Ok(code)
    }

    async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
