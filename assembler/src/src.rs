use core::fmt;
use std::path::PathBuf;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Src(pub PathBuf);

impl fmt::Display for Src {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
