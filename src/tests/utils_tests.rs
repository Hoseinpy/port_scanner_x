use crate::{
    constants::LOCALHOST_IP_ADDRESS,
    utils::{get_version, host_to_ip, translator},
};

#[tokio::test]
async fn test_host_to_ip_success() {
    let result = host_to_ip(String::from("tronhub.io")).await;
    assert!(result.is_ok())
}

#[tokio::test]
async fn test_host_to_ip_wrong_host() {
    let result = host_to_ip(String::from("https://slkjdflkjsdf.io")).await;
    assert!(result.is_err())
}

#[tokio::test]
async fn test_translator_host_none() {
    let some_addr = LOCALHOST_IP_ADDRESS;
    let result = translator(None, some_addr).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), some_addr)
}

#[tokio::test]
async fn test_translator_with_host() {
    let result = translator(Some(String::from("tronhub.io")), LOCALHOST_IP_ADDRESS).await;

    assert!(result.is_ok());
}

#[test]
fn test_get_version() {
    let result = get_version();
    assert!(!result.is_empty())
}
