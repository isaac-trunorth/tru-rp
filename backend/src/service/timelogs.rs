use chrono::{Datelike, Duration, NaiveDate, Weekday};
use entity::{dto::TimelogRequest, sea_orm_active_enums::Status, time_entries};
use migration::sea_orm::sea_query::Expr;
use migration::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use migration::Alias;

// todo: Add test for begin/end date
pub async fn get_timelogs(
    filters: &TimelogRequest,
    db: &DatabaseConnection,
) -> Vec<entity::time_entries::Model> {
    let mut logs = time_entries::Entity::find();

    // add the optional filters
    if let Some(user_id) = &filters.user_id {
        logs = logs.filter(time_entries::Column::EmployeeId.eq(user_id));
    }
    if let Some(manager_id) = &filters.user_id {
        logs = logs.filter(time_entries::Column::EmployeeId.eq(manager_id));
    }
    if let Some(status) = &filters.status {
        logs = logs.filter(time_entries::Column::SubmitStatus.eq(*status));
    };
    if let Some(date) = filters.week_end_date {
        let (begin_date, end_date) = get_date_range(date);
        logs = logs.filter(time_entries::Column::DateOfWork.between(begin_date, end_date));
    }

    let logs = logs
        .order_by_asc(time_entries::Column::DateOfWork)
        .all(db)
        .await
        .unwrap();
    logs
}

pub async fn mark_approved(db: &DatabaseConnection, ids: Vec<i32>) {
    time_entries::Entity::update_many()
        .col_expr(
            time_entries::Column::SubmitStatus,
            Expr::value(Status::Approved).cast_as(Alias::new("Status")),
        )
        .filter(time_entries::Column::Id.is_in(ids))
        .exec(db)
        .await
        .expect("update failed");
}

fn get_date_range(date: NaiveDate) -> (NaiveDate, NaiveDate) {
    let year = date.year_ce().1.try_into().unwrap();
    let mon = NaiveDate::from_isoywd_opt(year, date.iso_week().week(), Weekday::Mon).unwrap();
    let num_days: i64 = i64::from(date.num_days_from_ce() - mon.num_days_from_ce());
    println!("num days: {num_days}");
    let begin_date = date - Duration::days(num_days);
    let end_date = date + Duration::days(6 - num_days);
    println!("getting from: {:?} to {:?}", begin_date, end_date);
    (begin_date, end_date)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_beg() {
        let date = NaiveDate::from_str("2024-1-1").unwrap();
        let beg_date = NaiveDate::from_str("2024-1-1").unwrap();
        let end_date = NaiveDate::from_str("2024-1-7").unwrap();
        let range = get_date_range(date);
        assert_eq!(range.0, beg_date);
        assert_eq!(range.1, end_date);
    }

    #[test]
    fn test_end() {
        let date = NaiveDate::from_str("2024-1-7").unwrap();
        let beg_date = NaiveDate::from_str("2024-1-1").unwrap();
        let end_date = NaiveDate::from_str("2024-1-7").unwrap();
        let range = get_date_range(date);
        assert_eq!(range.0, beg_date);
        assert_eq!(range.1, end_date);
    }

    #[test]
    fn test_mid() {
        let date = NaiveDate::from_str("2024-1-5").unwrap();
        let beg_date = NaiveDate::from_str("2024-1-1").unwrap();
        let end_date = NaiveDate::from_str("2024-1-7").unwrap();
        let range = get_date_range(date);
        assert_eq!(range.0, beg_date);
        assert_eq!(range.1, end_date);
    }
}
