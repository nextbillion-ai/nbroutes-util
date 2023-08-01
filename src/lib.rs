pub mod coord;
pub mod def;
pub mod jwks;
pub mod osrm_path;
pub mod poly;
pub mod protos;
pub mod statsd;
pub mod util;

use chrono::prelude::*;
use def::{Engine, ValhallaError, OsrmError, AdaptError, EngineError};

use crate::coord::{Coord, Locatable};
use crate::osrm_path::get_data_root;
use crate::poly::load as load_poly;
use crate::util::load_maaas_area_config;
use geo::Polygon;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use util::Area;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Borders {
    pub area_list: Vec<Area>,
}

impl Borders {
    pub async fn populate_time_dependant_setting(&mut self, namespace: &Option<String>) {
        for area_setting in self.area_list.iter_mut() {
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

                    let mut filename = area_setting.name.to_owned();
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

#[derive(Deserialize, Clone, Debug)]
pub struct DaysAheadSlotSetting {
    pub id: String,
    pub range: Vec<u32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DaysAheadDaySetting {
    pub prefix: String,
    pub slots: Vec<DaysAheadSlotSetting>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DaysAheadSettting {
    pub timezone: f64,
    pub days: Vec<DaysAheadDaySetting>,
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Deserialize, Clone, Debug)]
pub struct RecurringDaySetting {
    pub name: String,
    pub prefix: String,
    pub days: Vec<RecurringDayDefinition>,
    pub slots: Vec<DaysAheadSlotSetting>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecurringSetting {
    pub timezone: f64,
    pub days: Vec<RecurringDaySetting>,
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Service {
    pub area: Area,
    pub mode: String,
}

pub fn find_area<'a>(
    mode: &Option<String>,
    coords: &Vec<Coord>,
    polygons: &HashMap<String, Vec<Polygon<f64>>>,
    areas: &'a Vec<Area>,
    tolerate_outlier: bool,
    request_id: Option<&str>,
    is_flexible_request: bool,
) -> Result<(&'a Area, String, Option<Vec<usize>>)> {
    let mut best_area = None;
    let mut best_coord_index = vec![];
    let mut mapped_mode: Option<String> = None;

    // the following two vars are used to keep track of the most likely areas
    //  in case there is no perfect match; so we can help user identify what coordinate is invalid
    let mut best_missing_coords = None;
    let mut best_number_of_coords = 0;

    for area in areas.iter() {
        let vs = polygons.get(area.name.as_str());
        if vs.is_none() {
            warn!("area name {} doesn't have polylgon", area.name.as_str());
            continue;
        }
        let vs = vs.unwrap();

        // coord_index stores the idx of coordinates that are in this area
        let mut coord_index = vec![];
        let mut missing_coords = vec![];
        for (idx, coord) in coords.iter().enumerate() {
            if coord.is_in_polygons(vs) {
                coord_index.push(idx);
                continue;
            }

            missing_coords.push(coord);
            if !tolerate_outlier {
                // early stop since we don't tolerate outlier
                break;
            }
            // continue to see how many coordinates actually is in this area
            continue;
        }

        if coord_index.len() == coords.len() {
            //     return here since we found an area that contains all points
            //      with the highest priority
            //      no need to return coord indexes since they're all in the area

            let mapped_mode_result = map_mode(mode, area, is_flexible_request);
            if mapped_mode_result.is_ok() {
                return Ok((area, mapped_mode_result.unwrap(), None));
            }
            continue;
        }

        if best_number_of_coords == 0 || coord_index.len() > best_number_of_coords {
            best_missing_coords = Some(missing_coords[0]);
            best_number_of_coords = coord_index.len();
        }

        if coord_index.len() == 0 {
            continue;
        }

        warn!(
            "some coordinates are not in area {:?}, coords: {:?}, request_id: {:?}",
            area.name, missing_coords, &request_id
        );

        if !tolerate_outlier {
            continue;
        }

        if coord_index.len() > best_coord_index.len() {
            let mapped_mode_result = map_mode(mode, area, is_flexible_request);
            if mapped_mode_result.is_ok() {
                best_area = Some(area);
                best_coord_index = coord_index;
                mapped_mode = Some(mapped_mode_result.unwrap());
            }
        }
    }

    if best_area.is_some() && mapped_mode.is_some() {
        return Ok((
            best_area.unwrap(),
            mapped_mode.unwrap(),
            Some(best_coord_index),
        ));
    }

    if best_missing_coords.is_some() {
        let best_missing_coords = best_missing_coords.unwrap();
        bail!("({},{})", best_missing_coords.lat(), best_missing_coords.lng());
    }

    bail!("")
}

pub fn find_service<'a>(
    mode: &Option<String>,
    coords: &'a Vec<Coord>,
    polygons: &HashMap<String, Vec<Polygon<f64>>>,
    areas: &Vec<Area>,
    tolerate_outlier: bool,
    request_id: Option<&str>,
    is_flexible_request: bool,
) -> Result<(Service, Option<Vec<usize>>)> {
    let (detected_area, mode, coord_index) =
        find_area(mode, coords, polygons, areas, tolerate_outlier, request_id, is_flexible_request)?;

    let r = Service {
        area: detected_area.clone(),
        mode: mode,
    };

    Ok((r, coord_index))
}

pub fn map_mode(mode: &Option<String>, area: &Area, is_flexible_request: bool) -> Result<String> {
    let mut default_mode = area.default_service.as_str();
    let mut mappings = &area.mappings;

    if is_flexible_request {
        if area.flexible_setting.is_none() {
            bail!("option=flexible not supported for this area")
        }
        
        let flexible_setting = area.flexible_setting.as_ref().unwrap();
        default_mode = flexible_setting.default_service.as_str();
        mappings = &flexible_setting.mappings;
    }

    if mode.is_some() && mode.as_ref().unwrap() != "" {
        match mappings.get(mode.as_ref().unwrap()) {
            Some(v) => return Ok(v.clone()),
            _ => {
                if mode.as_ref().unwrap().as_str() == default_mode {
                    return Ok(default_mode.to_string());
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

    if default_mode == "" {
        bail!("area not supported")
    }

    Ok(default_mode.to_string())
}

pub async fn load_polygons(
    areas: &HashSet<String>,
    skip_maaas: bool,
) -> Option<HashMap<String, Vec<Polygon<f64>>>> {
    if areas.len() == 0 {
        return None;
    }
    let mut maaas_area_cfg = load_maaas_area_config().await.ok();
    if !skip_maaas && maaas_area_cfg.is_none() {
        panic!("failed to load area defs from maaas");
    }
    let data_root = get_data_root();
    let mut polygons = HashMap::<String, Vec<Polygon<f64>>>::new();
    for area_name in areas {
        if !skip_maaas {
            let ps = maaas_area_cfg
                .as_mut()
                .unwrap()
                .polygons(area_name.as_str());
            if ps.is_some() {
                polygons.insert(area_name.clone(), ps.unwrap().to_vec());
                info!("loaded poly file from maaas-area-cfg for {}", &area_name);
                continue;
            }
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

pub fn handle_error_message(
    engine: &str,
    code: &str,
    message: &str
) -> String {
    match engine_mode_input(engine) {
        Engine::OSRM => error_handle_osrm(code, message),
        Engine::Valhalla => error_handle_valhalla(code, message),
    }
}

fn engine_mode_input(engine: &str) -> Engine {
    match engine {
        "osrm" => Engine::OSRM,
        _ => Engine::Valhalla,
    }
}

fn error_handle_valhalla(code: &str, message: &str) -> String{
    let error_type = match code {
        "Bad Request" => ValhallaError::BadRequest,
        "Not Implemented" => ValhallaError::NotImplemented,
        "Not Found" => ValhallaError::NotFound,
        "Method Not Allowed" => ValhallaError::MethodNotAllowed,
        "Internal Server Error" => ValhallaError::InternalServerError,
        "InvalidUrl" => ValhallaError::InvalidUrl,
        "InvalidService" =>  ValhallaError::InvalidService,
        "InvalidOptions" => ValhallaError::InvalidOptions,
        "InvalidValue" => ValhallaError::InvalidValue,
        "NoRoute" => ValhallaError::NoRoute,
        "NoSegment" => ValhallaError::NoSegment,
        "ServiceUnavailable" => ValhallaError::ServiceUnavailable, 
        "DistanceExceeded" => ValhallaError::DistanceExceeded,
        "PerimeterExceeded" => ValhallaError::PerimeterExceeded,
        "BreakageDistanceExceeded" => ValhallaError::BreakageDistanceExceeded,
        _ => ValhallaError::UnknownError,
    };
    handle_valhalla_err_message(error_type, message)
}

fn error_handle_osrm(code: &str, message: &str) -> String{
    let error_type = match code {
        "TooBig" => OsrmError::TooBig,
        "NotImplemented" => OsrmError::NotImplemented,
        "InvalidOptions" => OsrmError::InvalidOptions,
        "NoSegment" => OsrmError::NoSegment,
        "NoTable" => OsrmError::NoTable,
        "InvalidValue" => OsrmError::InvalidValue,
        "NoMatch" => OsrmError::NoMatch,
        "NoTrips" => OsrmError::NoTrips,
        "NoRoute" => OsrmError::NoRoute,
        _ => OsrmError::UnknownError,
    };
    handle_osrm_err_message(error_type, message).to_string()
}

fn handle_valhalla_err_message(error_type: ValhallaError, message: &str) -> String{
    // TODO: @Youzhi specific error
    let msg = match error_type {
        ValhallaError::BadRequest => match adapt_err_message(message){
            EngineError::InputNoPath => AdaptError::OutputRouteFailed,
            _ => AdaptError::OutputUnclassifiedError,
        },
        ValhallaError::NotImplemented => match adapt_err_message(message){
            _ => AdaptError::OutputNotImplemented,
        },
        ValhallaError::MethodNotAllowed => AdaptError::OutputMethodNotAllowed,
        ValhallaError::InternalServerError => AdaptError::OutputInternalServerError,
        ValhallaError::InvalidUrl => match adapt_err_message(message){
            _ => AdaptError::OutputInvalidUrl,
        },
        ValhallaError::NoSegment => AdaptError::OutputNoSegment,
        ValhallaError::InvalidOptions => match adapt_err_message(message){
            _ => AdaptError::OutputInvalidOption,
        },
        ValhallaError::NoRoute => AdaptError::OutputRouteFailed,
        ValhallaError::InvalidValue => match adapt_err_message(message){
            EngineError::InputFailedToParseLocation | EngineError::InputFailedToParseSource | EngineError::InputFailedToParseTarget | 
            EngineError::InputInsufficientLocations |  EngineError::InputInsufficientLocationsOrSourcesTargets | EngineError::InputInsufficientLocationsProvided |  
            EngineError::InputInsufficientSourcesProvided | EngineError::InputInsufficientTargetsProvided => AdaptError::OutputInvalidLocation,
            _ => AdaptError::OutputInvalidValue,
        },
        ValhallaError::DistanceExceeded | ValhallaError::PerimeterExceeded | ValhallaError::BreakageDistanceExceeded => AdaptError::OutputTooBig,
        _ => AdaptError::OutputUnclassifiedError,
    };
    msg.to_string()
}

fn handle_osrm_err_message(error_type: OsrmError, message: &str) -> String{
    // TODO: @Youzhi specific error
    let msg = match error_type {
        OsrmError::NoRoute => AdaptError::OutputRouteFailed,
        OsrmError::InvalidOptions => match adapt_err_message(message) {
            EngineError::InputCoordinatesInvalid => AdaptError::OutputCoordinatesInvalid,
            _ => AdaptError::OutputInvalidOption,
        },
        OsrmError::TooBig => match adapt_err_message(message){
            _ => AdaptError::OutputTooBig,
        },
        OsrmError::NotImplemented => match adapt_err_message(message){
            _ => AdaptError::OutputNotImplemented
        },
        OsrmError::NoSegment => match adapt_err_message(message){
            _ => AdaptError::OutputNoSegment,
        },
        OsrmError::NoTable => match adapt_err_message(message){
            EngineError::InputInvalidInputTable => AdaptError::OutputNoTableNode,
            _ => AdaptError::OutputNoTable,
        },
        OsrmError::InvalidValue => match adapt_err_message(message){
            _ => AdaptError::OutputInvalidValue,
        },
        OsrmError::NoMatch => match adapt_err_message(message){
            _ => AdaptError::OutputNoMatch,
        },
        OsrmError::NoTrips => def::AdaptError::OutputNoTrips,
        _ => AdaptError::OutputUnclassifiedError,
    };
    msg.to_string()
}

fn adapt_err_message(message: &str) -> EngineError {
    // TODO: @Youzhi specific error
    match message{
        "Failed to parse json request" => EngineError::InputFailedToParseJsonRequest,
        "No path could be found for input" => EngineError::InputNoPath,
        "Failed to parse location" => EngineError::InputFailedToParseLocation,
        "Failed to parse source" => EngineError::InputFailedToParseSource,
        "Failed to parse target" => EngineError::InputFailedToParseTarget,
        "Insufficiently specified required parameter 'locations'" => EngineError::InputInsufficientLocations,
        "Insufficiently specified required parameter 'locations' or 'sources & targets'" => EngineError::InputInsufficientLocationsOrSourcesTargets,
        "Insufficient number of locations provided" => EngineError::InputInsufficientLocationsProvided,
        "Insufficient number of sources provided" => EngineError::InputInsufficientSourcesProvided,
        "Insufficient number of targets provided" => EngineError::InputInsufficientTargetsProvided,
        "No table found, no valid input node" => EngineError::InputInvalidInputTable,
        _ => EngineError::InputUnknown,
    }
}