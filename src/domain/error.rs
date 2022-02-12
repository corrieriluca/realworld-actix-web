use actix_web::HttpResponse;
use serde::Serialize;

/// Returns a [`HttpResponse`], ready to be returned by a request
/// handler in case of **validation** error with the appropriate body.
/// The HTTP Status Code is set to 422 Unprocessable Entity, and the body
/// is a JSON-encoded [`ErrorResponse`].
pub fn validation_error(body: &str) -> HttpResponse {
    HttpResponse::UnprocessableEntity().json(ErrorResponse::new(body))
}

#[derive(Serialize)]
/// Error response model sent by any handler in case of error
pub struct ErrorResponse<'a> {
    errors: ErrorBody<'a>,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    body: Vec<&'a str>,
}

impl<'a> ErrorResponse<'a> {
    /// Create a new [`ErrorResponse`]
    pub fn new(error: &'a str) -> Self {
        ErrorResponse {
            errors: ErrorBody { body: vec![error] },
        }
    }
}
