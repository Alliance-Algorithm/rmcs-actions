use poem::Route;
use poem_openapi::{
    OpenApi, OpenApiService,
    payload::{Json, PlainText},
};

mod constant;
mod env;
mod logger;
mod meta;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get")]
    async fn pong(&self) -> PlainText<&'static str> {
        PlainText("pong")
    }

    #[oai(path = "/version", method = "get")]
    async fn version(&self) -> Json<meta::version::Version> {
        Json(meta::version::Version::default())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::load_env()?;
    logger::init_logger()?;

    let api_service = OpenApiService::new(Api, "RMCS Actions Service", "1.0")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/swagger", ui);

    poem::Server::new(poem::listener::TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
