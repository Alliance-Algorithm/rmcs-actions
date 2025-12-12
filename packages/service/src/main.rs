use poem::{EndpointExt, Route, get};
use poem_openapi::OpenApiService;

use crate::api::Api;

mod api;
mod constant;
mod env;
mod logger;
mod utils;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::load_env()?;
    logger::init_logger()?;

    let api_service = OpenApiService::new(Api, "RMCS Actions Service", "1.0")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/swagger", ui)
        .at(
            "/ws/:robot_id",
            get(service::websocket_service
                .data(tokio::sync::broadcast::channel::<String>(32).0)),
        );

    poem::Server::new(poem::listener::TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
