pub mod create_timelog {
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
}

pub mod delete_timelog {
    pub async fn soft_delete_timelog() {}
}
pub mod get_all_timelogs {
    use crate::app_state::AppState;
    use axum::{extract::State, Json};
    use entity::time_entries;
    use migration::sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

    pub async fn get_all_timelogs(state: State<AppState>) -> Json<Vec<time_entries::Model>> {
        let logs = time_entries::Entity::find()
            .filter(time_entries::Column::EmployeeId.eq(1))
            .order_by_asc(time_entries::Column::WeekEndDate)
            .all(&state.db)
            .await
            .unwrap();
        logs.into()
    }
}
pub mod get_one_timelog {
    pub async fn get_one_timelog() {}
}
pub mod update_timelogs {
    pub async fn mark_completed() {}
    pub async fn mark_uncompleted() {}
    pub async fn update_timelog() {}
}
