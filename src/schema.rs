// @generated automatically by Diesel CLI.

diesel::table! {
    facilities (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        price -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reservation_statuses (id) {
        id -> Unsigned<Bigint>,
        reservation_id -> Integer,
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reservations (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        facility_id -> Integer,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Bigint>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    facilities,
    reservation_statuses,
    reservations,
    users,
);
