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
    future::{ready, Ready},
    ops::Deref,
    rc::Rc,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web::Data,
    Error, FromRequest, HttpMessage,
};
use futures::{future::LocalBoxFuture, FutureExt};
use sqlx::PgPool;

use crate::{
    domain::auth::{decode_token, JwtSecret},
    repositories::user_repository::{get_user_by_username, User},
};

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

/// The implementation of the authentication middleware
pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone the Rc pointer so we can move it into the async block
        let service = Rc::clone(&self.service);

        async move {
            // Perform authentication logic and validation:

            // 1. Retrieve the DB Pool and the JWT secret
            let (pool, jwt_secret) = match (
                req.app_data::<Data<PgPool>>(),
                req.app_data::<Data<JwtSecret>>(),
            ) {
                (Some(p), Some(j)) => (p, j),
                _ => {
                    return Err(ErrorInternalServerError(
                        "Cannot access to internal resources.",
                    ))
                },
            };

            // 2. Extract the token from the request's Authorization header
            let token = match req.headers().get("Authorization") {
                Some(auth_header) => match auth_header.to_str() {
                    Ok(auth_header) => match auth_header.strip_prefix("Token") {
                        Some(token) => token.trim().to_owned(),
                        None => {
                            return Err(ErrorUnauthorized(
                                "Invalid format for Authorization header.",
                            ))
                        },
                    },
                    Err(_) => {
                        return Err(ErrorUnauthorized(
                            "Invalid format for Authorization header.",
                        ))
                    },
                },
                None => return Err(ErrorUnauthorized("No Authorization header found.")),
            };

            // 3. Decode the token and associate an user to it
            match decode_token(&token, &jwt_secret.0) {
                Ok(claims) => match get_user_by_username(pool, claims.username()).await {
                    Ok(user) => {
                        req.extensions_mut().insert::<AuthenticationInfo>(Rc::new(
                            AuthenticationResult { token, user },
                        ));
                    },
                    Err(_) => return Err(ErrorUnauthorized("Invalid token.")),
                },
                Err(_) => return Err(ErrorUnauthorized("Invalid token.")),
            }

            // Call the next service
            let fut = service.call(req);

            // Send the (unmodified) response
            let res = fut.await?;

            Ok(res)
        }
        .boxed_local()
    }
}

/// Request extractor that extracts an authentication result from the request's
/// extensions. This extractor **must** be used in conjunction with the
/// [`Authentication`] middleware as it extracts the [`AuthenticationInfo`] in
/// the request's extensions. Otherwise it will result in an Unauthorized error
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
            // Usually None is returned is the authentication middleware has not
            // been used.
            None => Err(ErrorUnauthorized(
                "You must provide an Authorization header.",
            )),
        };

        ready(result)
    }
}

/// Implements Deref trait for easier retrieving of the inner type of
/// [`AuthenticationInfo`].
impl Deref for AuthenticatedUser {
    type Target = AuthenticationInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
