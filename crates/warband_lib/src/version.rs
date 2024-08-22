#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Semver {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl std::fmt::Display for Semver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub const GIT_VERSION: &str = ""; // git_version::git_version!();

lazy_static::lazy_static! {
    pub static ref VERSION: Semver = Semver {
        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
    };
}

pub fn version() -> &'static str {
    use const_format::concatcp;
    concatcp!(
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR"),
        ".",
        env!("CARGO_PKG_VERSION_PATCH"),
        "+",
        GIT_VERSION,
    )
}
