// @generated automatically by Diesel CLI.

table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        completed -> Bool,
    }
}
