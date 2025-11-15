use crate::types::GlobalResult;
use colored::Colorize;
use std::io::Write;
use std::net::IpAddr;
use tokio::net;

pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            let level = record.level().as_str();
            let formatted_level = match level {
                "INFO" => level.bright_blue().bold(),
                "WARN" => level.bright_yellow().bold(),
                "ERROR" => level.red().bold(),
                _ => level.white(),
            };
            writeln!(buf, "[{}] : {}", formatted_level, record.args())
        })
        .filter_level(log::LevelFilter::Info)
        .init();
}

pub async fn host_to_ip(host: String) -> GlobalResult<IpAddr> {
    match net::lookup_host(format!("{}:{}", &host, 80)).await {
        Ok(mut ips) => {
            if let Some(ip) = ips.next() {
                Ok(ip.ip())
            } else {
                Err(format!("No IP address found for {}", host).into())
            }
        }
        Err(err) => Err(format!("Failed to lookup {}: {}", host, err).into()),
    }
}

pub async fn translator(host: Option<String>, address: IpAddr) -> GlobalResult<IpAddr> {
    if host.is_some() {
        log::info!("Translating host...");
        Ok(host_to_ip(host.unwrap()).await?)
    } else {
        Ok(address)
    }
}
