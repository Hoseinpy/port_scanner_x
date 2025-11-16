use colored::Colorize;
use futures::future;
use port_check::is_port_reachable_with_timeout;
use std::{net::IpAddr, sync::Arc, time::Duration};
use tokio::{sync::Semaphore, task::JoinHandle};

#[derive(Debug)]
pub struct Scanner {
    pub start_port: u16,
    pub end_port: u16,
    pub address: IpAddr,
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

        let semaphore = Arc::new(Semaphore::new(200));
        let mut tasks: Vec<JoinHandle<()>> = vec![];
        let address = self.address;

        log::info!("{}", "------------RESULT------------".green().bold());
        for port in self.start_port..self.end_port {
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let task = tokio::spawn(async move {
                let _permit = permit;
                let target = format!("{}:{}", address, port);

                if is_port_reachable_with_timeout(&target, Duration::from_millis(100)) {
                    log::info!("{target}");
                }
            });

            tasks.push(task);
        }

        future::join_all(tasks).await;
    }
}
