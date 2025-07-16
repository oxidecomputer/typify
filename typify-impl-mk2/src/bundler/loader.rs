use std::{collections::BTreeMap, fs::read_to_string, path::PathBuf};

use url::Url;

use crate::bundler::{LoadError, Loader};

pub struct NullLoader;

impl Loader for NullLoader {
    fn load(&self, url: url::Url) -> Result<String, LoadError> {
        Err(LoadError(format!(
            "no loader configured; unable to load {url}"
        )))
    }
}

#[derive(Default)]
pub struct FileMapLoader {
    files: BTreeMap<Url, PathBuf>,
}

impl FileMapLoader {
    pub fn add(mut self, url: Url, path: PathBuf) -> Self {
        self.files.insert(url, path);
        self
    }
}

impl Loader for FileMapLoader {
    fn load(&self, url: Url) -> Result<String, LoadError> {
        let Some(path) = self.files.get(&url) else {
            return Err(LoadError(format!("no mapping for {}", url)));
        };

        read_to_string(path).map_err(|e| LoadError(format!("error loading {}: {}", url, e)))
    }
}
