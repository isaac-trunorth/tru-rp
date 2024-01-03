pub mod create_user {

    use axum::{extract::State, Json};
    use entity::{managers, users};
    use migration::sea_orm::{
        ActiveModelTrait, ActiveValue::NotSet, ActiveValue::Set, EntityTrait, IntoActiveModel,
    };

    use crate::app_state::AppState;

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

    pub async fn create_manager(
        state: State<AppState>,
        Json(payload): Json<users::Model>,
    ) -> Result<String, String> {
        println!("{:?}", payload);
        managers::ActiveModel {
            id: NotSet,
            user_id: Set(payload.id),
        }
        .insert(&state.db)
        .await
        .expect("db insert failed");
        Ok("Success".into())
    }
    pub async fn create_user(
        state: State<AppState>,
        Json(payload): Json<users::Model>,
    ) -> Result<Json<entity::users::Model>, String> {
        let new_user = users::ActiveModel {
            id: NotSet,
            name: Set(payload.name),
            password: Set(payload.password),
            manager_id: NotSet,
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
