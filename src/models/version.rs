pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub rgx: Rgx,
}

impl Version {
    /// Convert a Version to a string.
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
    /// Attempts to parse a version string into a `Version` struct.
    /// If it fails, returns it will return `None`.
    /// In case it succeeds, returns the parsed version.
    /// If the string does no follow this: ```major.minor.patch``` then it will fail.
    pub fn from_string(s: &str) -> Option<Self> {
        let version_vec: Vec<&str> = s.split(".").collect();

        let rgx: Rgx = if version_vec.len() == 4 {
            Rgx::from_str(version_vec[3])
        } else {
            Rgx::ExactMatch
        };
        if version_vec.len() != 3 {
            return None;
        } else {
            Some(Self {
                major: match version_vec[0].parse::<u32>() {
                    Ok(m) => m,
                    _ => return None,
                },
                minor: match version_vec[1].parse::<u32>() {
                    Ok(m) => m,
                    _ => return None,
                },
                patch: match version_vec[2].parse::<u32>() {
                    Ok(p) => p,
                    _ => return None,
                },
                rgx,
            })
        }
    }
}

pub enum Rgx {
    Above,
    Below,
    ExactMatch,
    RangeTo(String),
}
impl Rgx {
    pub fn from_str(s: &str) -> Self {
        match s {
            "^" => Self::Above,
            "-" => Self::Below,
            _ => match Version::from_string(s) {
                Some(v) => Self::RangeTo(v.to_string()),
                None => Self::ExactMatch,
            },
        }
    }
}
