use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

// curl -i  --request POST 'http://localhost:8000/subscriptions' -d "email=foo%40gmail.com&name=Peter"

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
  println!("subscribe {} {}", form.email, form.name);

  match sqlx::query!(
    r#"
    insert into subscriptions (id, email, name, subscribed_at)
    values ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  )
  .execute(pool.get_ref())
  .await
  {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(e) => {
      println!("Failed to execute query {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
