use std::{env, net::SocketAddr};

use app_state::AppState;
use auth::RequireAuth;
use axum::{
    middleware::from_extractor,
    routing::{delete, get, patch, post, put},
    Router,
};
use migration::{sea_orm::Database, Migrator, MigratorTrait};
use routes::{
    timelogs::{
        create_timelog::create_timelog,
        delete_timelogs::soft_delete_timelog,
        get_timelogs::get_all_timelogs,
        update_timelogs::{mark_completed, mark_uncompleted, update_timelog},
    },
    users::{create_user::create_user, login::login, logout::logout},
};

mod app_state;
mod auth;
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

fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/users/logout", post(logout))
        .route("/timelogs", post(create_timelog))
        .route("/timelogs", get(get_all_timelogs))
        .route("/timelogs/:timelog_id/completed", put(mark_completed))
        .route("/timelogs/:timelog_id/uncompleted", put(mark_uncompleted))
        .route("/timelogs/:timelog_id", patch(update_timelog))
        .route("/timelogs/:timelog_id", delete(soft_delete_timelog))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route_layer(from_extractor::<RequireAuth>())
        .with_state(app_state)
}
