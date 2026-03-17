use poem::{EndpointExt, Route, get, middleware::Cors};
use poem_openapi::OpenApiService;

use crate::api::{Api, action::ActionApi, ident::IdentApi, stats::StatsApi};
use crate::constant::env::{DEFAULT_BIND_ADDR, ENV_NAME_BIND_ADDR};

mod api;
mod constant;
mod database;
mod env;
mod logger;
mod service;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::load_env()?;
    logger::init_logger()?;

    // Initialize database before starting the server
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = database::Database::new(&database_url).await?;
    db.init().await?;
    database::DATABASE
        .set(db)
        .map_err(|_| anyhow::anyhow!("Failed to set database"))?;

    let bind_addr = std::env::var(ENV_NAME_BIND_ADDR)
        .unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_string());

    let cors = Cors::new()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let server_url = format!("http://{bind_addr}/api");
    let api_service = OpenApiService::new(
        (Api, ActionApi, IdentApi, StatsApi),
        "RMCS Actions Service",
        "1.0",
    )
    .server(&server_url);
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/swagger", ui)
        .at(
            "/ws/:robot_uuid",
            get(service::websocket_service
                .data(tokio::sync::broadcast::channel::<String>(32).0)),
        )
        .with(cors);

    log::info!("Starting server on {}", bind_addr);
    poem::Server::new(poem::listener::TcpListener::bind(&bind_addr))
        .run(app)
        .await?;

    Ok(())
}
