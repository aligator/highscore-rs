use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

fn example_timestamp() -> &'static str {
    "2023-09-11T19:49:20.000000000Z"
}

fn example_i64() -> i64 {
    42
}

fn example_string() -> String {
    "foo".to_string()
}

#[derive(JsonSchema, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct NewHighscoreDTO {
    #[schemars(example = "example_string")]
    pub name: String,

    #[schemars(example = "example_i64")]
    pub score: i64,
}

#[derive(JsonSchema, Serialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct HighscoreDTO {
    pub id: i32,

    #[schemars(example = "example_string")]
    pub name: String,

    #[schemars(example = "example_i64")]
    pub score: i64,

    // schemars doesn't support the time crate yet, so pass it as string... (or use chrono...)
    /// Timestamp of the highscore entry as ISO8601 string.
    #[schemars(example = "example_timestamp")]
    pub created_at: String,
}
