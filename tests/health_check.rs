use dacha_landing::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};
use tokio::sync::oneshot::Sender;

fn spawn_app() -> (String, Sender<()>) {
    let (addr, tx, server) = run();
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
        .header("user-agent", "reqwest")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    tx.send(()).unwrap();
}

#[tokio::test]
async fn customer_registration_returns_a_400_when_data_is_missing() {
    let (address, tx) = spawn_app();
    let test_cases = vec![
        ("name=Joshua", "missing the email"),
        ("name=joshua@domain.com", "missing the name"),
        ("", "missing both email and name"),
    ];
    for (bad_body, message) in test_cases {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/register", address))
            .header("user-agent", "reqwest")
            .body(bad_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "The API did not fail with client error when payload was {}",
            message
        );
    }
    tx.send(()).unwrap();
}

#[tokio::test]
async fn customer_registration_returns_a_200_for_valid_form_data() {
    let (address, tx) = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let body = "name=Joshua&email=joshua@domain.com";
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/register", address))
        .header("user-agent", "reqwest")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let saved = sqlx::query!("SELECT email, name FROM customers",)
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription.");

    assert_eq!(saved.email, "joshua@domain.com");
    assert_eq!(saved.name, "joshua");
    tx.send(()).unwrap();
}
