use std::fs::File;
use serde::{Serialize, Deserialize};

// Handles mirrors
pub struct MirrorHandle {
    mirrors_dir: String,
}

impl MirrorHandle {
    pub fn new() -> Self {
        Self {
            mirrors_dir: std::env::var_os("BPS_MIRROR_ROOT")
                .expect("Cannot get mirror root from env variable: BPS_MIRROR_ROOT by default it should be: \"/var/lib/pkg/sync/\"")
                .to_str()
                .expect("BPS_MIRROR_ROOT cannot be parsed as a str")
                .to_string(),
        }
    }
    pub fn read_mirrors(&self) -> Vec<Mirror> {
        todo!()
    }
    pub fn find_pkg(&self, mirror: Mirror) {

    }
}

#[derive(Deserialize, Serialize)]
pub struct Mirror {
    name: String,
    url: String,
}