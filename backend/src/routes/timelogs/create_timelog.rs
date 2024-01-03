use axum::{extract::State, Json};
use entity::time_entries;
use migration::sea_orm::{ActiveValue::NotSet, EntityTrait, IntoActiveModel};

use crate::app_state::AppState;

pub async fn create_timelog(
    state: State<AppState>,
    Json(time_model): Json<Vec<time_entries::Model>>,
) -> String {
    let active = time_model
        .iter()
        .map(|mdl| {
            let mut actv = mdl.to_owned().into_active_model();
            actv.id = NotSet;
            actv
        })
        .collect::<Vec<time_entries::ActiveModel>>();
    let created = time_entries::Entity::insert_many(active)
        .exec(&state.db)
        .await;
    if created.is_ok() {
        "Successful".into()
    } else {
        created.expect_err("").to_string()
    }
}
