table! {
    organizations (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        _status -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
