use entity::dto::IdList;

use axum::{
    extract::{Path, State},
    Json,
};
use entity::time_entries;
use futures::future::join_all;
use migration::sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, EntityTrait, IntoActiveModel, Set,
};

use crate::{app_state::AppState, service};

pub async fn mark_approved(state: State<AppState>, Json(ids): Json<IdList>) -> String {
    service::timelogs::mark_approved(&state.db, ids.ids).await;
    "Success".into()
}

pub async fn update_timelog(
    state: State<AppState>,
    Json(time_model): Json<Vec<time_entries::Model>>,
) -> String {
    join_all(time_model.iter().map(|entry| {
        let active = entry.clone().into_active_model().reset_all();
        active.clone().into_active_model().update(&state.db)
    }))
    .await;
    return "Success".into();
}
