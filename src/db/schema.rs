use diesel::{table};

table! {
    user {
        id -> Integer,
        username  -> Text,
        password -> Text,
        enable -> Integer,
        createTime ->Timestamp,
    }
}
