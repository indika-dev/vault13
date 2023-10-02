use std::io::{BufRead, Error, ErrorKind, Result};

pub mod dat;
pub mod inifile;
pub mod stdfs;

#[derive(Clone, Debug)]
pub struct Metadata {
    len: u64,
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.len
    }
}

pub struct FileSystem {
    providers: Vec<Box<dyn Provider>>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            providers: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn Provider>) {
        self.providers.push(provider);
    }

    pub fn reader(&self, path: &str) -> Result<Box<dyn BufRead + Send>> {
        self.find_provider(path, |p| p.reader(path))
    }

    pub fn metadata(&self, path: &str) -> Result<Metadata> {
        self.find_provider(path, |p| p.metadata(path))
    }

    fn find_provider<T>(&self, path: &str, f: impl Fn(&dyn Provider) -> Result<T>) -> Result<T> {
        let mut error: Option<Error> = None;
        for provider in &self.providers {
            match f(provider.as_ref()) {
                Ok(r) => return Ok(r),
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        continue;
                    }
                    if error.is_none() {
                        error = Some(e);
                    }
                    break;
                }
            }
        }
        Err(error.unwrap_or_else(|| {
            Error::new(ErrorKind::NotFound, format!("file not found: {}", path))
        }))
    }

    pub fn exists(&self, path: &str) -> bool {
        self.metadata(path).is_ok()
    }
}

pub trait Provider {
    fn reader(&self, path: &str) -> Result<Box<dyn BufRead + Send>>;
    fn metadata(&self, path: &str) -> Result<Metadata>;
}
