#![allow(non_snake_case)]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const STATUS_OK: &str = "Ok";
pub const STATUS_FAILED: &str = "Failed";

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub enum GeometryInput {
    #[serde(rename = "polyline")]
    Polyline,
    #[serde(rename = "polyline6")]
    Polyline6,
}

// wrapper type to keep consistent with python api
#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct IntValue {
    pub value: u64,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct KeyInput {
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct DirectionsInput {
    #[doc = r"(origin: lat,lng) ^[\d\.\-]+,[\d\.\-]+$"]
    pub origin: String,
    #[doc = r"(destination: lat,lng) ^[\d\.\-]+,[\d\.\-]+$"]
    pub destination: String,
    #[doc = r"(waypoints: lat0,lng0|lat1,lng1|...) ^[\d\.\-]+,[\d\.\-]+(\|[\d\.\-]+,[\d\.\-]+)*$"]
    pub waypoints: Option<String>,
    #[doc = r"Default: false"]
    pub steps: Option<bool>,
    #[doc = r#"Default: """#]
    pub mode: Option<String>,
    #[doc = r#"departure_time: unix timestamp"#]
    pub departure_time: Option<i64>,
    #[doc = r#"(session: session id identifies the journey)"#]
    pub session: Option<String>,
    #[doc = r#"Default: polyline6"#]
    pub geometry: Option<GeometryInput>,
    #[doc = r#"Default: 1"#]
    pub altcount: Option<i32>,
    #[doc = r#"Default: false"#]
    pub alternatives: Option<bool>,
    #[doc = r#"Default: false"#]
    pub debug: Option<bool>,
    #[doc = r#"Default: """#]
    pub context: Option<String>,
    pub key: Option<String>,
    #[doc = r"(special_object_types: type1,type2,...)"]
    pub special_object_types: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct PostTripRouteInput {
    #[doc = r"(waypoints: lat0,lng0|lat1,lng1|...) ^[\d\.\-]+,[\d\.\-]+(\|[\d\.\-]+,[\d\.\-]+)*$"]
    pub waypoints: String,
    #[doc = r"(timestamps(in seconds): ts0|ts1|...) ^[\d]+(\|[\d]+)*$"]
    pub timestamps: Option<String>,
    #[doc = r#"Default: ["traffic_signals"]"#]
    pub special_object_types: Option<Vec<String>>,
    #[doc = r#"Default: """#]
    pub mode: Option<String>,
    #[doc = r#"Default: false"#]
    pub debug: Option<bool>,
    #[doc = r#"Default: """#]
    pub context: Option<String>,
    pub key: Option<String>,
    pub tolerate_outlier: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct PostTripRouteOutput {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    pub route: Option<MeteredRoute>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct DirectionsOutput {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    pub routes: Vec<Route>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct DirectionsTableOutput {
    pub status: String,
    #[serde(rename = "errorMessage")]
    pub error_msg: Option<String>,
    pub results: HashMap<String, DirectionsOutput>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct SpecialObject {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub coordinates: Location,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct MeteredRoute {
    #[doc = r#"Format: Polyline(https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"#]
    pub geometry: String,
    pub distance: f64,
    pub special_objects: Option<HashMap<String, Vec<SpecialObject>>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct Route {
    #[doc = r#"Format: Polyline(https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"#]
    pub geometry: Option<String>,
    pub distance: f64,
    pub duration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_location: Option<Location>,
    pub legs: Option<Vec<Leg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_objects: Option<HashMap<String, Vec<SpecialObject>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Apiv2Schema)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct Leg {
    pub distance: IntValue,
    pub duration: IntValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_duration: Option<IntValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct Step {
    pub geometry: Option<String>,
    pub start_location: Location,
    pub end_location: Location,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetNearbyInput {
    #[doc = r"(currentlocation: lat,lng) ^[\d\.\-]+,[\d\.\-]+$"]
    pub currentlocation: String,
    pub servicetype: String,
    #[doc = r#"Default: 10000"#]
    pub searchradius: Option<i64>,
    #[doc = r#"Default: 10"#]
    pub maxcount: Option<usize>,
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetNearbyOutput {
    pub status: String,
    pub msg: Option<String>,
    pub currentLocation: Location,
    pub searchRadius: i64,
    pub maxCount: usize,
    pub serviceType: String,
    pub results: Vec<NearbyResult>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct NearbyResult {
    pub id: String,
    pub location: Location,
    pub eta: u64,
    pub distance: u64,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MatrixInput {
    #[doc = r"(origins: lat0,lng0|lat1,lng1|...) ^[\d\.\-]+,[\d\.\-]+(\|[\d\.\-]+,[\d\.\-]+)*$"]
    pub origins: String,
    #[doc = r"(destinations: lat0,lng0|lat1,lng1|...) ^[\d\.\-]+,[\d\.\-]+(\|[\d\.\-]+,[\d\.\-]+)*$"]
    pub destinations: String,
    #[doc = r#"Default: """#]
    pub mode: Option<String>,
    #[doc = r#"departure_time: unix timestamp"#]
    pub departure_time: Option<i64>,
    #[doc = r#"Default: false"#]
    pub debug: Option<bool>,
    pub key: Option<String>,
    #[doc = r#"Default: """#]
    pub context: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MatrixOutput {
    pub status: String,
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Row {
    pub elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Element {
    pub duration: IntValue,
    pub distance: IntValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_duration: Option<IntValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_duration: Option<IntValue>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct SnapInput {
    #[doc = r"(path: lat0,lng0|lat1,lng1|...) ^[\d\.\-]+,[\d\.\-]+(\|[\d\.\-]+,[\d\.\-]+)*$"]
    pub path: String,
    #[doc = r"(timestamps(in seconds): ts0|ts1|...) ^[\d]+(\|[\d]+)*$"]
    pub timestamps: Option<String>,
    #[doc = r"(radiuses(in meters): radius0|radius1|...) ^[\d]+(\|[\d]+)*$"]
    pub radiuses: Option<String>,
    #[doc = r#"Default: false"#]
    pub interpolate: Option<bool>,
    pub key: Option<String>,
    #[doc = r#"Default: """#]
    pub context: Option<String>,
    pub tolerate_outlier: Option<bool>,
}

#[derive(Serialize, Deserialize, Apiv2Schema, Debug)]
pub struct SnapOutput {
    pub status: String,
    #[serde(rename = "snappedPoints")]
    pub snapped_points: Vec<SnappedPoint>,
    pub distance: u64,
    pub geometry: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Apiv2Schema, Debug)]
pub struct SnappedPoint {
    pub location: Location,
    #[serde(rename = "originalIndex")]
    pub original_index: u64,
    pub distance: f64,
    pub name: String,
    pub bearing: f64,
}
