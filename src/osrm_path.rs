use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var as env;
pub fn get_data_root() -> String {
    std::env::var("DATA_PATH").unwrap_or("/osrm".to_string())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsrmPaths {
    pub mappings: HashMap<String, OsrmPath>,
    pub env_path: Option<String>,
    pub data_root: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsrmPath {
    pub path: String,
    pub ts: u64,
}

impl OsrmPaths {
    pub fn load() -> Result<OsrmPaths> {
        let path = env("ENV_PATH").unwrap_or("/etc/env/config.yaml".to_string());
        let contents = std::fs::read_to_string(&path)?;
        let mut o: OsrmPaths = serde_yaml::from_str(&contents)?;
        o.env_path = Some(path);
        o.data_root = Some(get_data_root());
        Ok(o)
    }

    pub fn reload(&mut self) -> Result<HashMap<String, String>> {
        let contents = std::fs::read_to_string(self.env_path.as_ref().unwrap())?;
        let o: OsrmPaths = serde_yaml::from_str(&contents)?;
        let mut r = HashMap::<String, String>::new();
        for (service, osrm_path) in self.mappings.iter_mut() {
            if o.mappings.contains_key(service) {
                let op = o.mappings.get(service).unwrap();
                if op.ts > osrm_path.ts {
                    osrm_path.ts = op.ts;
                    osrm_path.path = op.path.clone();
                    r.insert(
                        service.to_string(),
                        _get(service, self.data_root.as_ref().unwrap(), &op.path).unwrap(),
                    );
                }
            }
        }
        Ok(r)
    }

    pub fn get(&self, service: &str) -> Option<String> {
        let mapping_path = match self.mappings.get(service) {
            Some(r) => &r.path,
            None => "missing",
        };
        _get(service, self.data_root.as_ref().unwrap(), mapping_path)
    }
}

fn _get(service: &str, data_root: &str, mapping_path: &str) -> Option<String> {
    match env(format!("{}_debug", &service.replace("-", "_"))) {
        Ok(r) => {
            return Some(r);
        }
        _ => {}
    }
    let p = format!("{}/{}", data_root, mapping_path);
    return Some(p);
}
