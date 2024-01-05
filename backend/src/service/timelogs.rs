use chrono::{Datelike, Duration, NaiveDate, Weekday};
use entity::time_entries;
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

    let (begin_date, end_date) = get_date_range(date);

    let logs = logs
        .filter(time_entries::Column::DateOfWork.between(begin_date, end_date))
        .order_by_asc(time_entries::Column::DateOfWork)
        .all(db)
        .await
        .unwrap();
    logs
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
