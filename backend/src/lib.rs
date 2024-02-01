use std::{env, net::SocketAddr};

use app_state::AppState;
use auth::RequireAuth;
use axum::{
    middleware::from_extractor,
    routing::{delete, get, post, put},
    Router,
};
use migration::{sea_orm::Database, Migrator, MigratorTrait};
use routes::{
    projects::{create_project, get_projects},
    timelogs::{
        create_timelog::create_timelog,
        delete_timelogs::soft_delete_timelog,
        get_timelogs::get_timelogs,
        update_timelogs::{mark_approved, update_timelog},
    },
    users::{assign_manager, create_user, get_users, get_users_by_manager, login, logout},
};
use tower_http::cors::CorsLayer;

mod app_state;
mod auth;
mod routes;
mod service;
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
        .route("/projects", get(get_projects))
        .route("/projects", post(create_project))
        .route("/timelogs", post(create_timelog))
        .route("/timelogs", put(update_timelog))
        .route("/timelogs", delete(soft_delete_timelog))
        .route("/timelogs/:user_id", get(get_timelogs))
        .route("/timelogs/approve", put(mark_approved))
        .route("/users", get(get_users))
        .route("/users/by_manager/:manager_id", get(get_users_by_manager))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
        .route("/users/setmanager", post(assign_manager))
        .route_layer(from_extractor::<RequireAuth>())
        .route("/login", post(login))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
}
