use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::daily_logs;

#[derive(Debug, Clone, Selectable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = daily_logs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DailyLog {
    pub id: String,
    pub content: String,
    pub log_date: String,
}
