use serde::{Deserialize, Serialize};

use super::{dependency::Dependency, version::Version};

#[derive(Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<Dependency>,
    pub description: Option<String>,
    pub package_script: String,
}
