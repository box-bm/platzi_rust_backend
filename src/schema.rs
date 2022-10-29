// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> BigInt,
        title -> Varchar,
        slug -> Varchar,
        body -> Text,
    }
}
