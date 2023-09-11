// @generated automatically by Diesel CLI.

diesel::table! {
    highscore (id) {
        id -> Integer,
        name -> Text,
        score -> BigInt,
        created_at -> Timestamp,
        game -> Text,
    }
}
