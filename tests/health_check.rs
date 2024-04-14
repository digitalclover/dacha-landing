use dacha_landing::run;

#[tokio::test]
async fn health_check_works() {
    let (server, addr, tx) = run();
    tokio::spawn(server);
    let client = reqwest::Client::new();
    let address = format!("http://{}/health_check", addr);
    let response = client
        .get(address)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    let _ = tx.send(());
}
