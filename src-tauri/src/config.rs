use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml::{self, Value};
use std::{collections::HashMap, fs, io, path::PathBuf, sync::RwLock};

/// Inner representation of the application configuration.
#[derive(Default)]
struct Config {
    path: PathBuf,
    data: HashMap<String, Value>,
}

impl Config {
    fn load(path: PathBuf) -> Self {
        let data = if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|contents| serde_yaml::from_str(&contents).ok())
                .unwrap_or_default()
        } else {
            HashMap::new()
        };
        Self { path, data }
    }

    fn save(&self) -> io::Result<()> {
        let contents = serde_yaml::to_string(&self.data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(&self.path, contents)
    }

    fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data
            .get(key)
            .and_then(|v| serde_yaml::from_value(v.clone()).ok())
    }

    fn set<T: Serialize>(&mut self, key: &str, value: T) -> io::Result<()> {
        let yaml_value =
            serde_yaml::to_value(value).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        self.data.insert(key.to_string(), yaml_value);
        self.save()
    }
}

static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.yaml");
    RwLock::new(Config::load(path))
});

/// Retrieve a single value from the configuration.
pub fn get_value<T: DeserializeOwned>(key: &str) -> Option<T> {
    CONFIG.read().ok().and_then(|cfg| cfg.get(key))
}

/// Store a single value in the configuration.
pub fn set_value<T: Serialize>(key: &str, value: T) -> io::Result<()> {
    CONFIG
        .write()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Lock poisoned"))?
        .set(key, value)
}

/// Retrieve a list of values from the configuration.
pub fn get_list<T: DeserializeOwned>(key: &str) -> Option<Vec<T>> {
    get_value(key)
}

/// Store a list of values in the configuration.
pub fn set_list<T: Serialize>(key: &str, list: Vec<T>) -> io::Result<()> {
    set_value(key, list)
}

/// Ensure that the configuration file is initialized.
pub fn init() {
    Lazy::force(&CONFIG);
}
