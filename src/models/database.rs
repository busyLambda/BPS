use super::package::Package;
use std::{fs, io::Read};

/// Handles the package database which keeps track of all installed software on the system.
/// It keeps track of their:
/// - Version number
/// - name
/// - dependencies
/// 
/// Why?
/// Because we want to look trough a smaller dataset when a user is e.g.: removing a program
/// In case we want to install a program we check if it is already installed on the system if not then we can look for it in the mirrors if we find it we contact the given url if it fails at any point then the user is promted with an error message.
pub struct Database {
    dbroot: String,
}

impl Database {
    pub fn new(dbroot: String) -> Database {
        Self {
            dbroot: std::env::var_os("BPS_DBROOT")
                .expect("Cannot get dbroot from env variable: BPS_DBROOT by default it should be: \"/var/lib/pkg/local/db/\"")
                .to_str()
                .expect("BPS_DBROOT cannot be parsed as a str")
                .to_string(),
        }
    }

    // TODO: Improve error handling.
    // FIXME: Return a result and an option maybe?
    pub fn read_pkg(&self, pkg_name: &str) -> Option<Package> {
        let mut file = match fs::File::open(format!("{}/{}.pkg", self.dbroot, pkg_name.to_string())) {
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
    pub fn write_pkg(&self, pkg: Package) -> Result<Package, WriteErr> {

        // TODO: Note that this is very simple, there should be some extra checks but this is ok for now.
        match fs::File::create(format!("{}/{}.pkg", self.dbroot, pkg.name)) {
            Ok(_) => return Ok(pkg),
            Err(err) => match err.kind() {
                ek => return Err(WriteErr::CannotWrite(ek))
            },
        };
    }
}

pub enum WriteErr {
    AlreadyWritten,
    WrittenButOutdated,
    CannotWrite(std::io::ErrorKind),
}