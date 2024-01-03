use crate::app_state::AppState;
use axum::{extract::State, Json};
use entity::time_entries;
use migration::sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

pub async fn get_all_timelogs(state: State<AppState>) -> Json<Vec<time_entries::Model>> {
    let logs = time_entries::Entity::find()
        .filter(time_entries::Column::EmployeeId.eq(1))
        .order_by_asc(time_entries::Column::DateOfWork)
        .all(&state.db)
        .await
        .unwrap();
    logs.into()
}
