use super::package::Package;
use std::{fs, io::Read};

pub struct Database {
    pub dbroot: String,
}

impl Database {
    pub fn new(dbroot: String) -> Database {
        Self {
            dbroot: std::env::var_os("BPS_DBROOT")
                .expect("Cannot get dbroot from env variable: BPS_DBROOT")
                .to_str()
                .expect("BPS_DBROOT cannot be parsed as a str")
                .to_string(),
        }
    }

    // TODO: Improve error handling.
    // FIXME: Return a result and an option maybe?
    pub fn read_pkg(&self, pkg_name: &str) -> Option<Package> {
        let mut file = match fs::File::open(format!("{}/{}.db", self.dbroot, pkg_name.to_string())) {
            Ok(f) => f,
            _ => return None,
        };
        let mut buf = String::new();

        match file.read_to_string(&mut buf) {
            Ok(_) => (),
            _ => return None,
        };

        match serde_json::from_str(&buf) {
            Ok(p) => return Some(p),
            _ => return None,
        };
    }
}
