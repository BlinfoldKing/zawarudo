table! {
    block (index) {
        index -> Int8,
        data -> Text,
        hash -> Text,
        prev_hash -> Text,
        created_at -> Timestamp,
        update_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    blocks (index) {
        index -> Int8,
        data -> Text,
        hash -> Text,
        prev_hash -> Text,
        created_at -> Timestamp,
        update_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    block,
    blocks,
);