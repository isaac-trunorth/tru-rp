use crate::{app_state::AppState, service};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDate;
use entity::time_entries;
use migration::sea_orm::{EntityTrait, QueryOrder};

pub async fn get_all_timelogs(state: State<AppState>) -> Json<Vec<time_entries::Model>> {
    let logs = time_entries::Entity::find()
        // .filter(time_entries::Column::EmployeeId.eq(1))
        .order_by_asc(time_entries::Column::DateOfWork)
        .all(&state.db)
        .await
        .unwrap();
    logs.into()
}

pub async fn get_initial_by_date(
    state: State<AppState>,
    Path(user_id): Path<i32>,
    date: String,
) -> Json<Vec<entity::time_entries::Model>> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
    let entries = service::timelogs::get_by_date(
        user_id,
        date,
        Some(entity::sea_orm_active_enums::Status::Initial),
        &state.db,
    )
    .await;
    entries.into()
}
