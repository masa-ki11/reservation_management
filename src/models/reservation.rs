use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;
use diesel::{Queryable, Identifiable, Associations, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User)]
pub struct Reservation {
    pub id: i32,
    pub user_id: i32,
    pub facility_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "reservations"]
pub struct NewReservation {
    pub user_id: i32,
    pub facility_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}