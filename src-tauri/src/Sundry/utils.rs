use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Content {
    pub motivation: String,
    pub means: String,
    pub side_effect: String,
}
