use serde::Deserialize;

use crate::sea_orm_active_enums::{Status, WorkCodes};
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimelogRequest {
    pub user_id: Option<i32>,
    pub manager_id: Option<String>,
    pub week_end_date: Option<chrono::NaiveDate>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub project_id: Option<i32>,
    pub status: Option<Status>,
    pub work_code: Option<WorkCodes>,
}

#[derive(Deserialize)]
pub struct IdList {
    pub ids: Vec<i32>,
}
