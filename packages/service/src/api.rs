pub mod action;
pub mod ident;
pub mod meta;
pub mod stats;

use poem::Error;
use poem_openapi::{
    ApiResponse, OpenApi,
    payload::{Json, PlainText},
};

use crate::service::{CONNECTIONS, instructions::Instruction};

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

fn bad_request(err: Error) -> GenericResponse {
    GenericResponse::InternalError(PlainText(format!("Bad request: {}", err)))
}

impl<T: std::error::Error> From<T> for GenericResponse {
    fn from(err: T) -> Self {
        GenericResponse::InternalError(PlainText(format!(
            "Internal error: {}",
            err
        )))
    }
}

pub type ApiResult<T> = Result<Json<T>, GenericResponse>;
pub type RawApiResult<T> = Result<T, GenericResponse>;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get")]
    async fn pong(&self) -> RawApiResult<PlainText<&'static str>> {
        Ok(PlainText("pong"))
    }

    #[oai(path = "/meta/version", method = "get")]
    async fn version(&self) -> ApiResult<meta::version::Version> {
        Ok(Json(meta::version::Version::default()))
    }
}
