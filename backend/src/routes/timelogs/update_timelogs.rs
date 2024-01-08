use axum::{extract::State, Json};
use entity::dto::ApprovedIds;

use crate::{app_state::AppState, service};

pub async fn mark_approved(state: State<AppState>, Json(ids): Json<ApprovedIds>) -> String {
    service::timelogs::mark_approved(&state.db, ids.ids).await;
    "Success".into()
}
pub async fn mark_initial() {}
pub async fn update_timelog() {}
