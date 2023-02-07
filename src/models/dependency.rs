use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}
