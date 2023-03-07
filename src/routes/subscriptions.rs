use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

// curl -i  --request POST 'http://localhost:8000/subscriptions' -d "email=foo%40gmail.com&name=Peter"

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
  println!("subscribe {} {}", form.email, form.name);
  HttpResponse::Ok().finish()
}
