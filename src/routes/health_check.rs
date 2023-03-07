use actix_web::HttpResponse;

// curl -i  --request GET 'http://localhost:8000/health_check'

pub async fn health_check() -> HttpResponse {
  // println!("health_check!!!");
  HttpResponse::Ok().finish()
}
