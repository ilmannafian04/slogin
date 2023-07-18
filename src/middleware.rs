use std::rc::Rc;

use actix_service::Transform;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    web, Error, HttpMessage,
};
use futures::{
    future::{ready, LocalBoxFuture, Ready},
    FutureExt,
};
use log::debug;
use sea_orm::DatabaseConnection;

use crate::{config::Config, entity::users, handler};

#[derive(Debug)]
pub struct AuthenticationInfo {
    pub user: Option<users::Model>,
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

// FOR THE LOVE OF GOD PLEASE IGNORE THIS MESS
impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        debug!("Start");
        let srv = self.service.clone();
        async move {
            match req.headers().get("Authorization") {
                Some(raw_jwt) => {
                    debug!("found jwt {:?}", raw_jwt);
                    let jwt = raw_jwt
                        .to_str()
                        .unwrap()
                        .to_string()
                        .strip_prefix("Bearer ")
                        .unwrap_or("")
                        .to_owned();
                    let config = req.app_data::<web::Data<Config>>().unwrap();
                    match handler::validate_jwt(jwt, &config.secret_key).await {
                        Some(token) => {
                            debug!("jwt is valid got token {:?}", token);
                            let db = req.app_data::<web::Data<DatabaseConnection>>().unwrap();
                            match handler::get_one_by_id(db, token).await {
                                Ok(user) => {
                                    debug!("got user {:?}", user);
                                    req.extensions_mut()
                                        .insert::<AuthenticationInfo>(AuthenticationInfo { user });
                                }
                                _ => {
                                    req.extensions_mut().insert::<AuthenticationInfo>(
                                        AuthenticationInfo { user: None },
                                    );
                                }
                            }
                        }
                        None => {
                            req.extensions_mut()
                                .insert::<AuthenticationInfo>(AuthenticationInfo { user: None });
                        }
                    };
                }
                None => {
                    req.extensions_mut()
                        .insert::<AuthenticationInfo>(AuthenticationInfo { user: None });
                }
            };

            Ok(srv.call(req).await?)
        }
        .boxed_local()
    }
}

pub struct AuthMiddlewareFactory {}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}
