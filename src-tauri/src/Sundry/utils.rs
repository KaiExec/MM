use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Content {
    pub motivation: String,
    pub means: String,
    pub side_effect: String,
}
