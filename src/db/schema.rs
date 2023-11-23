use diesel::{table};

table! {
    user {
        id -> Integer,
        name -> Text,
        phone -> Text,
        passwd -> Text,
    }
}
