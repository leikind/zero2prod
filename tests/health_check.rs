use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn health_check_works() {
  let address = spawn_app();

  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health_check", address))
    .send()
    .await
    .expect("failed to execute request");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn can_connect_to_db() {
  let _app_address = spawn_app();
  let configuration = get_configuration().expect("Failed to read configuration");
  let connection_string = configuration.database.connection_string();

  let _connection = PgConnection::connect(&connection_string)
    .await
    .expect("Failed to connect to Postgres");
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
  let app_address = spawn_app();
  let configuration = get_configuration().expect("Failed to read configuration");
  let connection_string = configuration.database.connection_string();

  let mut connection = PgConnection::connect(&connection_string)
    .await
    .expect("Failed to connect to Postgres");

  let client = reqwest::Client::new();
  let body = "name=yuri&email=yuri%40foo.org";

  let response = client
    .post(&format!("{}/subscriptions", &app_address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("select email, name from subscriptions")
    .fetch_one(&mut connection)
    .await
    .expect("Failed to fetch subscriptions");

  assert_eq!(saved.email, "yuri@foo.org");
  assert_eq!(saved.name, "yuri");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  let address = spawn_app();

  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=yuri", "missing the email"),
    ("email=yuri%40leikind.org", "missing the name"),
    ("", "missing the email and the name"),
  ];

  for (invalid_body, error_message) in test_cases {
    let response = client
      .post(&format!("{}/subscriptions", &address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalid_body)
      .send()
      .await
      .expect("Failed to execute request");

    assert_eq!(
      400,
      response.status().as_u16(),
      "The API did not fail with 400 when the payload was {}",
      error_message
    );
  }
}

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
  let port = listener.local_addr().unwrap().port();

  let server = zero2prod::startup::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  // println!("port {}", port);

  format!("http://127.0.0.1:{}", port)
}
