use actix_web::{get, web, HttpResponse};

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
