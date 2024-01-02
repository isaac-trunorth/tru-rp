use std::{env, net::SocketAddr};

use app_state::AppState;
use migration::{sea_orm::Database, Migrator, MigratorTrait};
use router::create_router;

mod app_state;
mod auth;
mod router;
mod routes;
mod utilities;

#[tokio::main]
pub async fn start() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let port = u16::from_str_radix(&port, 10).expect("PORT was invalid");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();
    let app_state: AppState = AppState { db: conn };
    let app = create_router(app_state);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
