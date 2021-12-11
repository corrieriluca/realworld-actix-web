use actix_web::{get, HttpResponse};

use crate::middlewares::Authentication;

#[get("", wrap = "Authentication")]
async fn user_info() -> HttpResponse {
    HttpResponse::Ok().body("Hello from user_info!")
}
