use crate::{app_state::AppState, service};
use axum::{extract::State, Json};
use entity::{dto::TimelogRequest, time_entries};

pub async fn get_timelogs(
    state: State<AppState>,
    Json(filters): Json<TimelogRequest>,
) -> Json<Vec<time_entries::Model>> {
    let logs = service::timelogs::get_timelogs(&filters, &state.db).await;
    logs.into()
}
