//! Module that contains the [`Authentication`] middleware.
//!
//! The [`Authentication`] middleware is responsible for granting access to
//! certain resources based on the JWT token passed in the Authorization header
//! of a incoming request.
//!
//! The incoming request's Authorization header must be of the form:
//! ```text
//! Authorization: Token jwt.token.here
//! ```
//! Note that this is not RFC-compliant.

use std::{
    future::{ready, Future, Ready},
    ops::Deref,
    pin::Pin,
    rc::Rc,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, FromRequest, HttpMessage,
};

use crate::models::users::User;

/// Struct for registering the authentication middleware.
pub struct Authentication;

/// Structure that holds the information of a succeeded authentication.
type AuthenticationInfo = Rc<AuthenticationResult>;

pub struct AuthenticationResult {
    /// The valid JWT token attached to this authentication.
    pub token: String,
    /// The user that is authentified.
    pub user: User,
}

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
        // TODO: Perform authentication logic and validation

        // Store the authentication result in the request's extensions
        req.extensions_mut()
            .insert::<AuthenticationInfo>(Rc::new(AuthenticationResult {
                token: "jwt.token.here".into(),
                user: User {
                    username: "username".into(),
                    email: "user@example.com".into(),
                    bio: None,
                    image: None,
                },
            }));

        // Call the next service
        let fut = self.service.call(req);

        // Send the (unmodified) response
        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}

/// Request extractor that extracts an authentication result from the request's
/// extensions. The extracted result must have been added by the
/// [`Authentication`] middleware, or it will result in a Unauthorized error
/// (header not present).
pub struct AuthenticatedUser(AuthenticationInfo);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<AuthenticationInfo>().cloned();
        let result = match value {
            Some(v) => Ok(AuthenticatedUser(v)),
            None => Err(ErrorUnauthorized(
                "You must provided an Authorization header.",
            )),
        };

        ready(result)
    }
}

/// Implements Deref trait for easier retrieving of the inner type of
/// [`Authenticated`].
impl Deref for AuthenticatedUser {
    type Target = AuthenticationInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
