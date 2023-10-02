use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::{Path, PathBuf},
};

use log::debug;

use super::{Metadata, Provider};

pub fn new_provider<P: AsRef<Path>>(path: P) -> Result<Box<dyn Provider>> {
    Ok(Box::new(Inifile::new(path)?))
}

#[derive(Debug)]
struct Inifile {
    path: PathBuf,
    state: State,
}

#[derive(Debug)]
enum State {
    Found,
    Missing,
    Err,
}

impl Inifile {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_file_result = File::open(path.as_ref());
        if config_file_result.is_ok() {
            Ok(Inifile {
                path: path.as_ref().to_path_buf(),
                state: State::Found,
            })
        } else {
            Ok(Inifile {
                path: path.as_ref().to_path_buf(),
                state: State::Missing,
            })
        }
    }
}

impl Provider for Inifile {
    fn reader(&self, path: &str) -> Result<Box<dyn BufRead + Send>> {
        debug!("loading ini file {}", path);
        let ini_reader = BufReader::new(File::open(path).unwrap());
        Ok(Box::new(ini_reader))
    }

    fn metadata(&self, path: &str) -> Result<Metadata> {
        // let len = self.to_fs_path(path).metadata()?.len();
        Ok(Metadata { len: 1 })
    }
}

pub fn normalize_path(path: &str) -> String {
    let mut r = String::with_capacity(path.len());

    for c in path.chars() {
        build_normalized_path(&mut r, Some(c));
    }
    build_normalized_path(&mut r, None);

    r
}

pub fn build_normalized_path(path: &mut String, c: Option<char>) {
    if let Some(mut c) = c {
        c = if c == '/' {
            '\\'
        } else {
            c.to_ascii_lowercase()
        };

        path.push(c);
    }

    if path == ".\\" || c.is_none() && path == "." {
        path.truncate(0);
    } else if path.ends_with("\\.\\") {
        let l = path.len();
        path.remove(l - 1);
        path.remove(l - 2);
    }
}
