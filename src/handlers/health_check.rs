use actix_web::{get, HttpResponse};

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
