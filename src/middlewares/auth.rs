//! Module that contains the [`Authentication`] middleware.

use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

/// The [`Authentication`] middleware is responsible for granting access to
/// certain resources based on the JWT token passed in the Authorization header
/// of a incoming request.
///
/// The incoming request's Authorization header must be of the form:
/// ```text
/// Authorization: Token jwt.token.here
/// ```
/// Note that this is not RFC-compliant.
pub struct Authentication;

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response status {}", res.status());
            Ok(res)
        })
    }
}
