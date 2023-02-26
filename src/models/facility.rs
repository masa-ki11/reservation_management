use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;
use diesel::{Queryable, Identifiable, Associations, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize)]
pub struct Facility {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "facilities"]
pub struct NewFacility {
    pub name: String,
    pub price: i32,
}