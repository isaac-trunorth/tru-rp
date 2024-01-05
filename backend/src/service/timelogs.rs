use std::ops::Sub;

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use entity::{sea_orm_active_enums::Status, time_entries};
use migration::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

// todo: Add test for begin/end date
pub async fn get_by_date(
    user_id: i32,
    date: NaiveDate,
    status: Option<entity::sea_orm_active_enums::Status>,
    db: &DatabaseConnection,
) -> Vec<entity::time_entries::Model> {
    let mut logs =
        time_entries::Entity::find().filter(time_entries::Column::EmployeeId.eq(user_id));

    // filter by status if requested
    if let Some(status) = status {
        logs = logs.filter(time_entries::Column::SubmitStatus.eq(status));
    };
    let year = date.year_ce().1.try_into().unwrap();
    let sun = NaiveDate::from_isoywd_opt(year, date.iso_week().week(), Weekday::Sun).unwrap();
    let num_days: i64 = i64::from(sun.num_days_from_ce() - date.num_days_from_ce());
    println!("num days: {num_days}");
    let begin_date = date - Duration::days(7 - num_days);
    let end_date = date + Duration::days(num_days - 1);
    println!("getting from: {:?} to {:?}", begin_date, end_date);

    let logs = logs
        .filter(time_entries::Column::DateOfWork.between(begin_date, end_date))
        .order_by_asc(time_entries::Column::DateOfWork)
        .all(db)
        .await
        .unwrap();
    logs
}
