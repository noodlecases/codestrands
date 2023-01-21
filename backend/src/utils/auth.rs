use std::pin::Pin;

use actix_session::SessionExt;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::Future;

use crate::utils::error::{codestrands_error, CodestrandsError};

pub struct UserSession {
    pub user_id: i32,
}

impl FromRequest for UserSession {
    type Error = CodestrandsError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();

        Box::pin(async move {
            if let Some(user_id) = session.get("user_id").unwrap_or(None) {
                if session
                    .get::<Option<bool>>("authenticated")
                    .unwrap_or(None)
                    .is_some()
                {
                    return Ok(Self { user_id });
                }
            }
            codestrands_error!(401, "unauthorized")
        })
    }
}
