use actix_web::{get, HttpResponse};

use crate::middlewares;

#[get("", wrap = "middlewares::Authentication")]
async fn user_info(aa: middlewares::AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().body(format!("token: {}\nuser: {}", aa.token, aa.user))
}
