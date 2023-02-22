// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
