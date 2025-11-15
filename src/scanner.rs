use futures::future;
use port_check::is_port_reachable_with_timeout;
use std::{net::IpAddr, time::Duration};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct Scanner {
    start_port: u16,
    end_port: u16,
    address: IpAddr,
}

impl Scanner {
    pub fn new(start_port: u16, end_port: u16, address: IpAddr) -> Self {
        Self {
            start_port,
            end_port,
            address,
        }
    }

    pub async fn scan(&self) {
        log::info!("Start scanning for: {}\n", self.address);

        let mut tasks: Vec<JoinHandle<()>> = vec![];
        let address = self.address;

        for port in self.start_port..self.end_port {
            let task = tokio::spawn(async move {
                if is_port_reachable_with_timeout(
                    format!("{}:{}", address, port),
                    Duration::from_millis(30),
                ) {
                    log::info!("Found: {}:{}", address, port);
                }
            });

            tasks.push(task);
        }

        future::join_all(tasks).await;
    }
}
