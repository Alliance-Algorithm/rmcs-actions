pub mod action;
pub mod ident;
pub mod meta;
pub mod stats;

use poem::Error;
use poem_openapi::{
    ApiResponse, OpenApi,
    payload::{Json, PlainText},
};
use serde::Deserialize;

#[derive(Debug, Clone, ApiResponse)]
#[oai(bad_request_handler = "bad_request")]
pub enum GenericResponse {
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

#[allow(clippy::needless_pass_by_value)]
fn bad_request(err: Error) -> GenericResponse {
    GenericResponse::BadRequest(PlainText(format!("Bad request: {err}")))
}

impl<T: Into<anyhow::Error>> From<T> for GenericResponse {
    fn from(err: T) -> Self {
        GenericResponse::InternalError(PlainText(format!(
            "Internal error: {}",
            err.into()
        )))
    }
}

pub type ApiResult<T> = Result<Json<T>, GenericResponse>;
pub type RawApiResult<T> = Result<T, GenericResponse>;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get")]
    #[allow(clippy::unused_async)]
    async fn pong(&self) -> RawApiResult<PlainText<&'static str>> {
        Ok(PlainText("pong"))
    }

    #[oai(path = "/meta/version", method = "get")]
    #[allow(clippy::unused_async)]
    async fn version(&self) -> ApiResult<meta::version::Version> {
        Ok(Json(meta::version::Version::default()))
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct AnyDeserialize {}
