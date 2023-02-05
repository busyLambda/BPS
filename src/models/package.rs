use super::{dependency::Dependency, version::Version};

pub struct Package {
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<Dependency>,
    pub description: Option<String>,
    pub package_script: String,
}
