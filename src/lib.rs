pub mod coord;
pub mod def;
pub mod jwks;
pub mod osrm_path;
pub mod poly;
pub mod statsd;
pub mod util;

use crate::coord::{Coord, Locatable};
use crate::osrm_path::get_data_root;
use crate::poly::load as load_poly;
use geo::Polygon;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate prometheus;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn timestamp() -> i64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Borders {
    pub areas: BTreeMap<String, Area>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Area {
    pub name: String,
    pub default_service: String,
    pub mappings: BTreeMap<String, String>,
}

#[derive(Clone)]
pub struct Service {
    pub area: String,
    pub mode: String,
    pub origin_area_conf: Area,
}

pub fn find_service<'a>(
    mode: &Option<String>,
    coords: &'a Vec<Coord>,
    polygons: &HashMap<String, Vec<Polygon<f64>>>,
    areas: &BTreeMap<String, Area>,
    tolerate_outlier: bool,
) -> Result<(Service, Option<Vec<usize>>)> {
    let mut detected = HashMap::<&String, Vec<usize>>::new();

    for (idx, coord) in coords.iter().enumerate() {
        let d = coord.locate(polygons);
        if d.is_err() {
            if tolerate_outlier {
                continue;
            }
            bail!(d.err().unwrap())
        }
        detected.entry(d?).or_insert(vec![]).push(idx);
    }

    let mut detected_area = None;
    let mut new_coord_indexes = None;
    match detected.len() {
        0 => bail!("not area is detected"),
        1 => {
            for (key, value) in detected.into_iter() {
                detected_area = Some(key);
                if value.len() != coords.len() {
                    new_coord_indexes = Some(value);
                }
                break;
            }
        }
        _ => {
            if !tolerate_outlier {
                bail!("more than one area is detected");
            }
            let mut best_area = None;
            let mut best_locations: Vec<usize> = vec![];
            for (area, locations) in detected.into_iter() {
                if best_area.is_none() || locations.len() > best_locations.len() {
                    best_area = Some(area);
                    best_locations = locations;
                }
            }
            detected_area = best_area;
            new_coord_indexes = Some(best_locations);
        }
    }

    let detected_area = detected_area.unwrap();

    if !areas.contains_key(detected_area) {
        warn!("area {} not found in config", detected_area);
        bail!("detected area not in config")
    }
    let area = areas.get(detected_area).unwrap();
    let mapped_mode = map_mode(mode, area.default_service.clone(), area)?;

    let r = Service {
        area: detected_area.clone(),
        mode: mapped_mode,
        origin_area_conf: area.clone(),
    };

    Ok((r, new_coord_indexes))
}

pub fn map_mode(mode: &Option<String>, default_mode: String, area: &Area) -> Result<String> {
    if mode.is_some() {
        match area.mappings.get(mode.as_ref().unwrap()) {
            Some(v) => return Ok(v.clone()),
            _ => {
                if mode.as_ref().unwrap().as_str() == default_mode.as_str() {
                    return Ok(default_mode);
                } else {
                    warn!(
                        "map_mode failed due to unknown mode: {}",
                        mode.as_ref().unwrap()
                    );
                    bail!("invalid mode input")
                }
            }
        }
    }

    Ok(default_mode)
}

// todo: fix the osrm path and data root later. currently gateway doesn't need osrmpaths
pub fn load_polygons(borders: &Option<Borders>) -> Option<HashMap<String, Vec<Polygon<f64>>>> {
    if borders.is_none() {
        return None;
    }
    let borders = borders.as_ref().unwrap();
    // let osrm_paths = OsrmPaths::load()?;
    let data_root = get_data_root();
    let mut polygons = HashMap::<String, Vec<Polygon<f64>>>::new();
    for area_name in borders.areas.keys() {
        polygons.insert(
            area_name.clone(),
            load_poly(&format!("{}/mojo/borders/{}.poly", data_root, &area_name))
                .expect(&format!("failed to load poly for {}", &area_name)),
        );
        info!("loaded poly file for {}", &area_name);
    }
    Some(polygons)
}
