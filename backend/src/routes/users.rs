pub mod create_user {

    use axum::{extract::State, Json};
    use entity::users;
    use migration::sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ActiveValue::Set};

    use crate::app_state::AppState;

    pub async fn create_user(
        state: State<AppState>,
        Json(payload): Json<users::Model>,
    ) -> Result<Json<entity::users::Model>, String> {
        let new_user = users::ActiveModel {
            id: NotSet,
            name: Set(payload.name),
            password: Set(payload.password),
        }
        .insert(&state.db)
        .await
        .expect("db insert failed");
        Ok(new_user.into())
    }
}
pub mod login {
    pub async fn login() {}
}
pub mod logout {
    pub async fn logout() {}
}
