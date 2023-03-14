use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
  let app = spawn_app().await;

  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health_check", app.address))
    .send()
    .await
    .expect("failed to execute request");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  let body = "name=yuri&email=yuri%40foo.org";

  let response = client
    .post(&format!("{}/subscriptions", &app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("select email, name from subscriptions")
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch subscriptions");

  assert_eq!(saved.email, "yuri@foo.org");
  assert_eq!(saved.name, "yuri");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=yuri", "missing the email"),
    ("email=yuri%40leikind.org", "missing the name"),
    ("", "missing the email and the name"),
  ];

  for (invalid_body, error_message) in test_cases {
    let response = client
      .post(&format!("{}/subscriptions", &app.address))
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

async fn spawn_app() -> TestApp {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let mut configuration = get_configuration().expect("Failed to read configuration");

  configuration.database.database_name = Uuid::new_v4().to_string();

  let connection_pool = configure_database(&configuration.database).await;

  let server =
    zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  // println!("port {}", port);

  TestApp {
    address,
    db_pool: connection_pool,
  }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
  let mut connection = PgConnection::connect(&config.connection_string_without_db())
    .await
    .expect("Failed to connect to Postgres");

  let q = format!(r#"CREATE DATABASE "{}";"#, config.database_name);

  connection
    .execute(q.as_str())
    .await
    .expect("Failed to create database.");

  let connection_pool = PgPool::connect(&config.connection_string())
    .await
    .expect("Failed to connect to Postgres.");

  sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database");

  connection_pool
}

// cargo test test_uuids -- --exact --nocapture
#[test]
fn test_uuids() {
  use uuid::Uuid;

  // let u: Uuid = Uuid::new_v4();
  let u: Uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").expect("Failed to parse UUID");

  assert_eq!(u.simple().to_string(), "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8");
  assert_eq!(
    u.hyphenated().to_string(),
    "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
  );
  assert_eq!(
    u.braced().to_string(),
    "{a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8}"
  );
  assert_eq!(
    u.urn().to_string(),
    "urn:uuid:a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
  );
}

// cargo test test_utc_lib -- --exact --nocapture
#[test]
fn test_utc_lib() {
  use chrono::Utc;

  let a = Utc::now();
  println!("{}", a);
}
