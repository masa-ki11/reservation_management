use chrono::{DateTime, Utc};
use diesel::{Queryable, Identifiable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User)]
#[table_name = "reservations"]
pub struct Reservation {
    pub id: i32,
    pub user_id: i32,
    pub facility_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, Serialize)]
#[table_name = "facilities"]
pub struct Facility {
    pub id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(Reservation)]
#[table_name = "reservation_statuses"]
pub struct ReservationStatus {
    pub id: i32,
    pub reservation_id: i32,
    pub status: String,
}

