table! {
    Event (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        data -> Nullable<Jsonb>,
        timestamp -> Timestamp,
        audit_sk -> Int4,
    }
}