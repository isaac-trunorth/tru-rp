use axum::{extract::State, Json};
use entity::projects;
use migration::sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ActiveValue::Set, EntityTrait, ModelTrait,
};

use crate::app_state::AppState;

pub async fn create_project(
    state: State<AppState>,
    Json(payload): Json<projects::Model>,
) -> Result<Json<entity::projects::Model>, String> {
    let new_user = projects::ActiveModel {
        id: NotSet,
        job_number: Set(payload.job_number),
        job_description: Set(payload.job_description),
        job_active: Set(payload.job_active),
    }
    .insert(&state.db)
    .await
    .expect("db insert failed");
    Ok(new_user.into())
}

pub async fn get_projects(state: State<AppState>) -> Json<Vec<entity::projects::Model>> {
    let projects = projects::Entity::find().all(&state.db).await.unwrap();
    projects.into()
}
