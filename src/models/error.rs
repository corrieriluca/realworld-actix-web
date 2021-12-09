use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    errors: ErrorBodies<'a>,
}

#[derive(Serialize)]
struct ErrorBodies<'a> {
    body: Vec<&'a str>,
}

impl<'a> ErrorResponse<'a> {
    /// Create a new [`ErrorResponse`]
    pub fn new(error: &'a str) -> Self {
        ErrorResponse {
            errors: ErrorBodies { body: vec![error] },
        }
    }
}
