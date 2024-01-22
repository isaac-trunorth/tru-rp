use serde::Deserialize;

use crate::sea_orm_active_enums::Status;
#[derive(Deserialize)]
pub struct TimelogRequest {
    pub user_id: Option<i32>,
    pub manager_id: Option<String>,
    pub week_end_date: Option<chrono::NaiveDate>,
    pub status: Option<Status>,
}

#[derive(Deserialize)]
pub struct IdList {
    pub ids: Vec<i32>,
}
