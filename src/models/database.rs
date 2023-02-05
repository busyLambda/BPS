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
}
