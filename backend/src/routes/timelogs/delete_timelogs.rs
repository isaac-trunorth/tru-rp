use axum::{
    extract::{Path, State},
    Json,
};
use entity::{dto::IdList, time_entries};
use futures::future::join_all;
use migration::sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, EntityTrait, IntoActiveModel, ModelTrait,
};

use crate::app_state::AppState;

pub async fn soft_delete_timelog(state: State<AppState>, Json(ids): Json<IdList>) -> String {
    join_all(
        ids.ids
            .iter()
            .map(|id| time_entries::Entity::delete_by_id(*id).exec(&state.db)),
    )
    .await;
    return "Success".into();
}
