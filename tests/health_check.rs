use std::net::TcpListener;

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
async fn subscribe_returns_a_200_for_valid_data() {
  let address = spawn_app();
  let client = reqwest::Client::new();
  let body = "name=yuri&email=yuri%40leikind.org";

  // println!("{}", &format!("{}/subscriptions", &address));

  let response = client
    .post(&format!("{}/subscriptions", &address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

  assert_eq!(200, response.status().as_u16());
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

  let server = zero2prod::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  // println!("port {}", port);

  format!("http://127.0.0.1:{}", port)
}
