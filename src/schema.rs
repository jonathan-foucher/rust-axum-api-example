// @generated automatically by Diesel CLI.

diesel::table! {
    movie (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        release_date -> Date,
    }
}
