//! The error handler is not really a request handler, it may be used by
//! other handlers to generate a [`HttpResponse`] in case of error.

use actix_web::HttpResponse;

use crate::models::error::ErrorResponse;

/// Returns a [`HttpResponse`], ready to be returned by a request
/// handler in case of **validation** error with the appropriate body.
/// The HTTP Status Code is set to 422 Unprocessable Entity, and the body
/// is a JSON-encoded [`ErrorResponse`].
pub fn validation_error(body: &str) -> HttpResponse {
    HttpResponse::UnprocessableEntity().json(ErrorResponse::new(body))
}
