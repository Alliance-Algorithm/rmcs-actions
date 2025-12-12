pub mod ident;
pub mod meta;

use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get")]
    async fn pong(&self) -> PlainText<&'static str> {
        PlainText("pong")
    }

    #[oai(path = "/meta/version", method = "get")]
    async fn version(&self) -> Json<meta::version::Version> {
        Json(meta::version::Version::default())
    }

    #[oai(path = "/ident/whoami", method = "post")]
    async fn whoami(
        &self,
        info: Json<ident::whoami::WhoAmI>,
    ) -> Json<ident::whoami::WhoAmIResponse> {
        // Placeholder implementation
        println!("Received WhoAmI info: {:?}", info);
        Json(ident::whoami::WhoAmIResponse {
            robot_id: format!("robot_{}:{}", info.username, info.mac),
        })
    }
}
