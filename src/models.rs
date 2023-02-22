use chrono::{DateTime, Utc};
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
