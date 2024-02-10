use crate::{app_state::AppState, service};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use entity::{dto::TimelogRequest, time_entries};

pub async fn get_timelogs(
    state: State<AppState>,
    filters: Query<TimelogRequest>,
) -> Json<Vec<time_entries::Model>> {
    let logs = service::timelogs::get_timelogs(&filters, &state.db).await;
    logs.into()
}
