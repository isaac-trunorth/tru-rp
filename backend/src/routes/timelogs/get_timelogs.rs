use crate::{app_state::AppState, service};
use axum::{
    extract::{Path, State},
    Json,
};
use entity::{dto::TimelogRequest, time_entries};

pub async fn get_timelogs(state: State<AppState>, id: Path<i32>) -> Json<Vec<time_entries::Model>> {
    let filters = TimelogRequest {
        user_id: Some(*id),
        manager_id: None,
        week_end_date: None,
        status: None,
    };
    let logs = service::timelogs::get_timelogs(&filters, &state.db).await;
    logs.into()
}
