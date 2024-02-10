use axum::{
    body::Body,
    extract::{path, Path, State},
    Json,
};
use entity::users;
use migration::sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ActiveValue::Set, ColumnTrait, EntityTrait,
    IntoActiveModel, QueryFilter,
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
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        user_name: Set(payload.user_name),
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
    let from_db = users::Entity::find()
        .filter(users::Column::UserName.eq(payload.user_name))
        .one(&state.db)
        .await;
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

pub async fn update_password(state: State<AppState>, Json(payload): Json<users::Model>) -> String {
    let user_pwd = hash_password(&payload.password);
    let active: users::ActiveModel = users::ActiveModel {
        id: Set(payload.id),
        manager_id: NotSet,
        first_name: NotSet,
        last_name: NotSet,
        user_name: NotSet,
        password: Set(user_pwd),
        access_level: NotSet,
    };
    active.update(&state.db).await.unwrap();
    return "Success".into();
}

pub async fn get_users(state: State<AppState>) -> Json<Vec<users::Model>> {
    let users = users::Entity::find().all(&state.db).await.unwrap();
    users.into()
}

pub async fn get_users_by_manager(
    state: State<AppState>,
    Path(manager_id): Path<i32>,
) -> Json<Vec<users::Model>> {
    let users = users::Entity::find()
        .filter(users::Column::ManagerId.eq(manager_id))
        .all(&state.db)
        .await
        .unwrap();
    users.into()
}
pub async fn update_user(state: State<AppState>, Json(user): Json<users::Model>) -> String {
    let active: users::ActiveModel = users::ActiveModel {
        id: Set(user.id),
        manager_id: Set(user.manager_id),
        first_name: Set(user.first_name),
        last_name: Set(user.last_name),
        user_name: Set(user.user_name),
        password: NotSet,
        access_level: Set(user.access_level),
    };
    active.update(&state.db).await.unwrap();
    return "Success".into();
}
