use core::fmt;
use std::{
    env::current_dir,
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

// this struct is just to allow formatting error messages using relative paths and to combine all
// src paths to a single struct
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Src(PathBuf);

impl Src {
    pub fn new(pathbuf: PathBuf) -> io::Result<Self> {
        let pathbuf = pathbuf.canonicalize()?;

        Ok(Src(pathbuf))
    }

    pub fn get_relative(&self, relative_path: &Path) -> io::Result<Src> {
        // should be safe to unwrap parent() here because it is canonical
        Src::new(self.parent().unwrap().join(relative_path))
    }
}

impl Deref for Src {
    type Target = PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Src {
    // print the path relative to current directory (if it can, otherwise just print the
    // original canonical path)
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // not sure if there is a better way, but need to have relative_path live long enough, so
        // creating a variable here
        let mut relative_path = None;
        if let Ok(cwd) = current_dir() {
            relative_path = Some(path_relative_from(self.0.as_path(), cwd.as_path())).flatten();
        }

        let path = match &relative_path {
            Some(relative_path) => relative_path.as_path(),
            None => self.0.as_path(),
        };

        write!(f, "{}", path.display())
    }
}

fn path_relative_from(path: &Path, base: &Path) -> Option<PathBuf> {
    use std::path::Component;

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(Component::CurDir)) => comps.push(a),
                (Some(_), Some(Component::ParentDir)) => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}
