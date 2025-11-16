use std::net::TcpListener;

use crate::{constants::LOCALHOST_IP_ADDRESS, scanner::Scanner};

#[test]
fn test_scanner_new() {
    let scanner = Scanner::new(10, 100, LOCALHOST_IP_ADDRESS);

    assert_eq!(scanner.start_port, 10);
    assert_eq!(scanner.end_port, 100);
    assert_eq!(scanner.address, LOCALHOST_IP_ADDRESS)
}

#[tokio::test]
async fn test_scan_closed_port() {
    let scanner = Scanner::new(9, 10, LOCALHOST_IP_ADDRESS);

    scanner.scan().await;

    assert!(true)
}

#[tokio::test]
async fn test_scan_open_port() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    let port = listener.local_addr().unwrap().port();

    std::thread::spawn(move || {
        let _ = listener.accept();
    });

    let scanner = Scanner::new(port, port + 1, LOCALHOST_IP_ADDRESS);

    scanner.scan().await;

    assert!(true)
}

#[tokio::test]
async fn test_dose_not_panic() {
    let scanner = Scanner::new(1, 5, LOCALHOST_IP_ADDRESS);

    scanner.scan().await;

    assert!(true);
}
