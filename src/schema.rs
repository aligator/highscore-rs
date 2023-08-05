// @generated automatically by Diesel CLI.

diesel::table! {
    highscore (id) {
        id -> Integer,
        name -> Text,
        score -> Float,
        created_at -> Nullable<Timestamp>,
    }
}
