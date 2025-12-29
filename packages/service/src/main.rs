use poem::{EndpointExt, Route, get, middleware::Cors};
use poem_openapi::OpenApiService;

use crate::api::{Api, action::ActionApi, ident::IdentApi, stats::StatsApi};

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

    let cors = Cors::new()
        .allow_origin("http://localhost:5173")
        .allow_methods(vec!["GET", "POST"])
        .allow_credentials(true);

    let api_service = OpenApiService::new(
        (Api, ActionApi, IdentApi, StatsApi),
        "RMCS Actions Service",
        "1.0",
    )
    .server("http://localhost:3000/api");
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

    poem::Server::new(poem::listener::TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
