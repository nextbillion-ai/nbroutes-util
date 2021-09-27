pub mod coord;
pub mod def;
pub mod jwks;
pub mod osrm_path;
pub mod poly;
pub mod protos;
pub mod statsd;
pub mod util;
use chrono::prelude::*;

use crate::coord::{Coord, Locatable};
use crate::osrm_path::get_data_root;
use crate::poly::load as load_poly;
use crate::util::load_maaas_area_config;
use geo::Polygon;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate prometheus;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn timestamp() -> i64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Borders {
    pub areas: BTreeMap<String, Area>,
}

impl Borders {
    pub async fn populate_time_dependant_setting(&mut self, namespace: &Option<String>) {
        for (area, area_setting) in self.areas.iter_mut() {
            if area_setting.time_dependant.is_none() {
                continue;
            }
            if namespace.is_none() {
                warn!("populate_time_dependant_setting fails since namespace is not configured");
                continue;
            }

            let ns = namespace.as_ref().unwrap().as_str();

            let mut area_time_dependant =
                BTreeMap::<String, BTreeMap<String, TimeDependantSetting>>::new();
            for (mode, mode_setting) in area_setting.time_dependant.as_ref().unwrap() {
                let mut mode_time_dependant = BTreeMap::<String, TimeDependantSetting>::new();

                for (ctx, enabled) in mode_setting {
                    if !enabled {
                        continue;
                    }

                    let mut filename = area.to_owned();
                    if ctx.as_str() != "" {
                        filename = filename + "-" + ctx.as_str();
                    }
                    filename = filename + "-" + mode.as_str();

                    let url = format!("https://storage.googleapis.com/static.nextbillion.io/nbroute/time_dependant_setting/{}/{}.yaml?{}", ns, filename.as_str(), timestamp());
                    let maybe_resp = reqwest::get(url.as_str()).await;
                    if maybe_resp.is_err() {
                        warn!("populate_time_dependant_setting fails to get setting for filename {} due to {:?}", &filename, maybe_resp.err().unwrap());
                        continue;
                    }
                    let maybe_body = maybe_resp.unwrap().text().await;
                    if maybe_body.is_err() {
                        warn!("populate_time_dependant_setting fails to get setting for filename {} due to {:?}", &filename, maybe_body.err().unwrap());
                        continue;
                    }
                    let body = maybe_body.unwrap();
                    let maybe_setting = serde_yaml::from_str(&body);
                    if maybe_setting.is_err() {
                        warn!("populate_time_dependant_setting fails to get setting for filename {} due to {:?}, contents: {}", &filename, maybe_setting.err().unwrap(), body.as_str());
                        continue;
                    }
                    mode_time_dependant.insert(ctx.clone(), maybe_setting.unwrap());
                }

                if mode_time_dependant.len() > 0 {
                    area_time_dependant.insert(mode.clone(), mode_time_dependant);
                }
            }

            if area_time_dependant.len() > 0 {
                area_setting.time_dependant_settings = Some(area_time_dependant);
            }
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct DaysAheadSlotSetting {
    pub id: String,
    pub range: Vec<u32>,
}

#[derive(Deserialize, Clone)]
pub struct DaysAheadDaySetting {
    pub prefix: String,
    pub slots: Vec<DaysAheadSlotSetting>,
}

#[derive(Deserialize, Clone)]
pub struct DaysAheadSettting {
    pub timezone: f64,
    pub days: Vec<DaysAheadDaySetting>,
}

#[derive(Deserialize, Clone)]
pub struct RecurringDayDefinition {
    pub day_type: String,
    pub date_value: Option<Vec<String>>,
    pub weekday_value: Option<Vec<u32>>,
}

impl RecurringDayDefinition {
    pub fn match_time(&self, target_date: &str, target_weekday: &Weekday) -> bool {
        return match self.day_type.as_str() {
            "date" => {
                if self.date_value.is_none() {
                    warn!("match_time missing date_value with day_type=date");
                    return false;
                }
                for v in self.date_value.as_ref().unwrap() {
                    if target_date == v.as_str() {
                        return true;
                    }
                }
                false
            }
            "weekday" => {
                if self.weekday_value.is_none() {
                    warn!("match_time missing weekday_value with day_type=weekday");
                    return false;
                }
                for v in self.weekday_value.as_ref().unwrap() {
                    if target_weekday.number_from_monday() - 1 == v.clone() {
                        return true;
                    }
                }
                debug!(
                    "match_time fails since target_weekday {} does not match {:?}",
                    target_weekday.number_from_monday() - 1,
                    self.weekday_value.as_ref().unwrap()
                );
                false
            }
            _ => {
                warn!("match_time invalid day_type: {}", self.day_type.as_str());
                false
            }
        };
    }
}

#[derive(Deserialize, Clone)]
pub struct RecurringDaySetting {
    pub name: String,
    pub prefix: String,
    pub days: Vec<RecurringDayDefinition>,
    pub slots: Vec<DaysAheadSlotSetting>,
}

#[derive(Deserialize, Clone)]
pub struct RecurringSetting {
    pub timezone: f64,
    pub days: Vec<RecurringDaySetting>,
}

#[derive(Deserialize, Clone)]
pub struct TimeDependantSetting {
    pub setting_type: String,
    pub days_ahead_setting: Option<DaysAheadSettting>,
    pub recurring_setting: Option<RecurringSetting>,
}

impl TimeDependantSetting {
    pub fn get_additional_ctx_days_ahead(&self, ts: i64) -> Option<String> {
        if self.days_ahead_setting.is_none() {
            warn!("days_ahead_setting is None");
            return None;
        }
        let days_ahead_setting = self.days_ahead_setting.as_ref().unwrap();

        let time_zone: FixedOffset;
        if days_ahead_setting.timezone >= 0.0 {
            time_zone = FixedOffset::east((days_ahead_setting.timezone * 3600.0) as i32);
        } else {
            time_zone = FixedOffset::west((-days_ahead_setting.timezone * 3600.0) as i32);
        }
        let time_now = Utc::now().with_timezone(&time_zone);
        let today_start_ts = time_zone
            .ymd(time_now.year(), time_now.month(), time_now.day())
            .and_hms_nano(0, 0, 0, 0)
            .timestamp();
        debug!("get_additional_ctx today_start_ts is {}", today_start_ts);

        let target_ts_since_today = ts - today_start_ts;
        if target_ts_since_today < 0 {
            debug!("get_additional_ctx returns None ts is before today");
            return None;
        }

        let days_since_today = target_ts_since_today / 86400;
        if days_since_today >= days_ahead_setting.days.len() as i64 {
            debug!("get_additional_ctx returns None ts is beyond plan");
            return None;
        }

        let seconds_since_target_day = target_ts_since_today - (days_since_today * 86400);
        let target_day = &days_ahead_setting.days[days_since_today as usize];
        for slot in target_day.slots.iter() {
            if seconds_since_target_day >= ((slot.range[0] * 3600) as i64)
                && seconds_since_target_day <= ((slot.range[1] * 3600) as i64)
            {
                return Some(target_day.prefix.to_owned() + slot.id.as_str());
            }
        }

        debug!("get_additional_ctx returns None since no slot is found for the day");
        None
    }

    pub fn get_additional_ctx_recurring(&self, ts: i64) -> Option<String> {
        if self.recurring_setting.is_none() {
            warn!("recurring_setting is None");
            return None;
        }
        let recurring_setting = self.recurring_setting.as_ref().unwrap();

        let time_zone: FixedOffset;
        if recurring_setting.timezone >= 0.0 {
            time_zone = FixedOffset::east((recurring_setting.timezone * 3600.0) as i32);
        } else {
            time_zone = FixedOffset::west((-recurring_setting.timezone * 3600.0) as i32);
        }

        // get target ts's time as local time
        // TODO: experiment whether this really work...
        let target_local_time =
            DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(ts, 0), time_zone);
        let target_date = format!(
            "{}/{}/{}",
            target_local_time.year(),
            target_local_time.month(),
            target_local_time.day()
        );
        let target_weekday = target_local_time.weekday();
        let target_hour = target_local_time.hour();
        debug!(
            "local time for ts {} is {:?} {} {}, {}",
            ts,
            &target_local_time,
            target_date.as_str(),
            target_weekday.number_from_monday() - 1,
            target_hour
        );

        for recurring_day in recurring_setting.days.iter() {
            for day in recurring_day.days.iter() {
                if !day.match_time(target_date.as_str(), &target_weekday) {
                    continue;
                }
                for slot in recurring_day.slots.iter() {
                    if slot.range.len() < 2 {
                        warn!(
                            "get_additional_ctx_recurring invalid slot range {:?}",
                            &slot.range
                        );
                        continue;
                    }
                    if target_hour < slot.range[0] {
                        continue;
                    }
                    if target_hour >= slot.range[1] {
                        continue;
                    }

                    return Some(recurring_day.prefix.to_owned() + slot.id.as_str());
                }
            }
        }

        debug!("get_additional_ctx returns None since no slot is found for the day");
        None
    }

    pub fn get_additional_ctx(&self, ts: i64) -> Option<String> {
        return match self.setting_type.as_str() {
            "days-ahead" => self.get_additional_ctx_days_ahead(ts),
            "recurring" => self.get_additional_ctx_recurring(ts),
            _ => {
                warn!(
                    "get_additional_ctx encouters invalid setting type: {}",
                    self.setting_type.as_str()
                );
                None
            }
        };
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Area {
    pub name: String,
    pub default_service: String,
    pub mappings: BTreeMap<String, String>,
    pub time_dependant: Option<BTreeMap<String, BTreeMap<String, bool>>>,
    #[serde(skip_deserializing, skip_serializing)]
    pub time_dependant_settings: Option<BTreeMap<String, BTreeMap<String, TimeDependantSetting>>>,
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
pub async fn load_polygons(
    borders: &Option<Borders>,
) -> Option<HashMap<String, Vec<Polygon<f64>>>> {
    if borders.is_none() {
        return None;
    }
    let mut maaas_area_cfg = load_maaas_area_config().await.unwrap();
    let borders = borders.as_ref().unwrap();
    // let osrm_paths = OsrmPaths::load()?;
    let data_root = get_data_root();
    let mut polygons = HashMap::<String, Vec<Polygon<f64>>>::new();
    for area_name in borders.areas.keys() {
        let ps = maaas_area_cfg.polygons(&area_name);
        if ps.is_some() {
            polygons.insert(area_name.clone(), ps.unwrap().to_vec());
            info!("loaded poly file from maaas-area-cfg for {}", &area_name);
            continue;
        }
        polygons.insert(
            area_name.clone(),
            load_poly(&format!("{}/mojo/borders/{}.poly", data_root, &area_name))
                .expect(&format!("failed to load poly for {}", &area_name)),
        );
        info!("loaded poly file for {}", &area_name);
    }
    Some(polygons)
}
