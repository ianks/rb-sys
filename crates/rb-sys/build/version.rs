use crate::RbConfig;

pub const LATEST_STABLE_VERSION: Version = Version::new(3, 2);
pub const MIN_SUPPORTED_STABLE_VERSION: Version = Version::new(2, 6);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct Version(u32, u32);

impl Version {
    pub const fn new(major: u32, minor: u32) -> Self {
        Self(major, minor)
    }

    pub fn major(&self) -> u32 {
        self.0
    }

    pub fn minor(&self) -> u32 {
        self.1
    }

    pub fn current(rbconfig: &RbConfig) -> Version {
        Self(
            rbconfig.get("MAJOR").parse::<i32>().unwrap() as _,
            rbconfig.get("MINOR").parse::<i32>().unwrap() as _,
        )
    }

    pub fn is_stable(&self) -> bool {
        *self >= MIN_SUPPORTED_STABLE_VERSION && *self <= LATEST_STABLE_VERSION
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}
