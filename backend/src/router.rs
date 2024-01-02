use crate::{
    app_state::AppState,
    auth::RequireAuth,
    routes::{
        hello_world::hello_world,
        timelogs::{
            create_timelog::create_timelog,
            delete_timelog::soft_delete_timelog,
            get_all_timelogs::get_all_timelogs,
            get_one_timelog::get_one_timelog,
            update_timelogs::{mark_completed, mark_uncompleted, update_timelog},
        },
        users::{create_user::create_user, login::login, logout::logout},
    },
};

use axum::{
    middleware::from_extractor,
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/users/logout", post(logout))
        .route("/timelogs", post(create_timelog))
        .route("/timelogs", get(get_all_timelogs))
        .route("/timelogs/:timelog_id", get(get_one_timelog))
        .route("/timelogs/:timelog_id/completed", put(mark_completed))
        .route("/timelogs/:timelog_id/uncompleted", put(mark_uncompleted))
        .route("/timelogs/:timelog_id", patch(update_timelog))
        .route("/timelogs/:timelog_id", delete(soft_delete_timelog))
        .route("/", get(hello_world))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route_layer(from_extractor::<RequireAuth>())
        .with_state(app_state)
}
