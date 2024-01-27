use axum::{extract::State, Json};
use entity::users;
use migration::sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ActiveValue::Set, EntityTrait, IntoActiveModel,
};

use crate::{
    app_state::AppState,
    auth::{create_token, hash_password},
};

pub async fn assign_manager(
    state: State<AppState>,
    Json(payload): Json<users::Model>,
) -> Result<String, String> {
    let user = users::Entity::find_by_id(payload.id)
        .one(&state.db)
        .await
        .expect("User not found")
        .unwrap();
    let mut user = user.into_active_model();
    user.manager_id = Set(payload.manager_id);
    user.update(&state.db).await.expect("Manager update failed");
    Ok("Success".into())
}

pub async fn create_user(
    state: State<AppState>,
    Json(payload): Json<users::Model>,
) -> Result<Json<entity::users::Model>, String> {
    let pwd = hash_password(&payload.password);
    let new_user = users::ActiveModel {
        id: NotSet,
        name: Set(payload.name),
        password: Set(pwd),
        manager_id: Set(payload.manager_id),
        access_level: Set(payload.access_level),
    }
    .insert(&state.db)
    .await
    .expect("db insert failed");
    Ok(new_user.into())
}
pub async fn login(state: State<AppState>, Json(payload): Json<users::Model>) -> String {
    let from_db = users::Entity::find_by_id(payload.id).one(&state.db).await;
    if from_db.is_err() {
        return "User not found".into();
    };
    let from_db = from_db.unwrap().unwrap();
    let user_pwd = hash_password(&payload.password);
    if user_pwd != from_db.password {
        return "Invalid password".into();
    };

    let token = create_token(from_db);
    if token.is_ok() {
        token.unwrap()
    } else {
        "Token creation failed".into()
    }
}
pub async fn logout() {}
