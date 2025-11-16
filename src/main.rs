use bpaf::Bpaf;
use std::net::IpAddr;

use crate::{
    constants::{LOCALHOST_IP_ADDRESS, MAX_PORT_RANGE},
    scanner::Scanner,
    utils::{get_version, init_logger, translator},
};

mod constants;
mod scanner;
mod types;
mod utils;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Arguments {
    #[bpaf(long("address"), short('a'), fallback(LOCALHOST_IP_ADDRESS))]
    address: IpAddr,
    #[bpaf(long("host"), short('h'), fallback(None))]
    host: Option<String>,
    #[bpaf(long("start"), short('s'), guard(|n| *n > 0, "start must grater than 0"), fallback(1u16))]
    start_port: u16,
    #[bpaf(long("end"), short('e'), fallback(MAX_PORT_RANGE))]
    end_port: u16,
}

#[tokio::main]
async fn main() {
    // init logger
    init_logger();

    log::info!("port_scanner_x version {} started!", get_version());

    let opts = arguments().run();
    let ip_address = translator(opts.host, opts.address)
        .await
        .unwrap_or_else(|error| {
            log::error!("{error}");
            std::process::exit(1)
        });

    let scanner = Scanner::new(opts.start_port, opts.end_port, ip_address);
    scanner.scan().await;
}
