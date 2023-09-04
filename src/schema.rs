// @generated automatically by Diesel CLI.

diesel::table! {
    highscore (id) {
        id -> Integer,
        name -> Text,
        score -> Integer,
        created_at -> Timestamp,
    }
}
