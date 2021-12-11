use serde::Serialize;

#[derive(Serialize)]
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
