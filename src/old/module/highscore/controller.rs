use rocket::post;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

struct Controller {
    highscore_service: service,
}

