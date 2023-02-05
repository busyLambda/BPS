use super::{version::Version, dependency::Dependency};

pub struct Package {
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<Dependency>,
    pub description: Option<String>,
    pub package_script: String,
}