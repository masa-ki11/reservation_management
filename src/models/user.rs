use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;
use diesel::{Queryable, Identifiable, Associations, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: Option<i64>,
}