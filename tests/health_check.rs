use dacha_landing::run;
use tokio::sync::oneshot::Sender;

fn spawn_app() -> (String, Sender<()>) {
    let (addr, tx, server) = run(0);
    println!("{}", addr);
    tokio::spawn(server);
    (format!("http://{}", addr), tx)
}

#[tokio::test]
async fn health_check_works() {
    let (address, tx) = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    tx.send(()).unwrap();
}
