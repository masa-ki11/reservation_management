use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;
use diesel::{Queryable, Identifiable, Associations, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Reservation)]
pub struct ReservationStatus {
    pub id: i32,
    pub reservation_id: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "reservation_statuses"]
pub struct NewReservationStatus {
    pub reservation_id: i32,
    pub status: String,
}