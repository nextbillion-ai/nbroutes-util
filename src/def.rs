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
    #[doc = "location of origin.\n\nFormat: `lat,lng`.\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
    pub origin: String,
    #[doc = "location of destination.\n\nFormat: `lat,lng`.\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
    pub destination: String,
    #[doc = "location(s) of waypoint(s) along the trip.\n\nFormat: `lat0,lng0|lat1,lng1|...`.\n\nRegex: (^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$)"]
    pub waypoints: Option<String>,
    #[doc = "enable to include `steps` in response.\n\nDefault: `false`"]
    pub steps: Option<bool>,
    #[doc = "mode of service.\n\nValues:`car|auto|bike|escooter|4w|2w...`.\n\nDefault: `\"\"`"]
    pub mode: Option<String>,
    #[doc = "departure time.\n\nFormat: `unix timestamp`.\n\nUnit: `seconds`.\n\nDefault: `0`"]
    pub departure_time: Option<i64>,
    #[doc = "unique session id for trip identification.\n\nNote: Help to reuse cached trip characteritics when set. \n\nDefault: `\"\"`"]
    pub session: Option<String>,
    #[doc = "output format of geometry.\n\nDefault: `polyline6`"]
    pub geometry: Option<GeometryInput>,
    #[doc = "number of alternative routes to return.\n\nDefault: `1` if `alternatives` is disabled, `3` otherwise"]
    pub altcount: Option<i32>,
    #[doc = "enable to return alternative routes.\n\nNote: `altcount` will default to `3` if this is disabled.\n\nDefault: `false`"]
    pub alternatives: Option<bool>,
    #[doc = "enable to show debug information.\n\nDefault: `false`"]
    pub debug: Option<bool>,
    #[doc = "`deprecated`"]
    pub context: Option<String>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
    #[doc = "special geospatial objects to include in response.\n\nFormat: `type1,type2,...`.\n\nDefault:`\"\"`"]
    pub special_object_types: Option<String>,
    #[doc = "`deprecated`"]
    pub annotations: Option<bool>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct PostTripRouteInput {
    #[doc = "location(s) of waypoint(s) along the trip.\n\nFormat:`lat0,lng0|lat1,lng1|...`\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$"]
    pub waypoints: String,
    #[doc = "unix timestamp of each `waypoints`.\n\nUnit: `seconds`\n\nFormat: `ts0|ts1|...`\n\nRegex: ^[\\d]+(\\|[\\d]+)*$"]
    pub timestamps: Option<String>,
    #[doc = "special geospatial objects to include in response.\n\nDefault: `[\"traffic_signals\"]`"]
    pub special_object_types: Option<Vec<String>>,
    #[doc = "mode of service.\n\nValues:`car|auto|bike|escooter|4w|2w...`.\n\nDefault: \"\""]
    pub mode: Option<String>,
    #[doc = "enable to show debug information.\n\nDefault: `false`"]
    pub debug: Option<bool>,
    #[doc = "`deprecated`"]
    pub context: Option<String>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
    #[doc = "enable to ignore location not found in service boundary.\n\nNote: enable this to ignore outliers, otherwise an error will be thrown.\n\nDefault: `false`"]
    pub tolerate_outlier: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct PostTripRouteOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "service mode used.\n\nValues:`4w|3w|2w...`."]
    pub mode: Option<String>,
    #[doc = "`route` calculated."]
    pub route: Option<MeteredRoute>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    #[doc = "error message when `status` != `Ok`"]
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct DirectionsOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "service mode used.\n\nValues:`4w|3w|2w...`."]
    pub mode: Option<String>,
    #[doc = "`routes` calculated."]
    pub routes: Vec<Route>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "special geospatial objects found from all `routes`.\n\nNote: this is super collection of `special_objects` from individual `route`"]
    pub global_special_objects: Option<HashMap<String, Vec<SpecialObject>>>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    #[doc = "error message when `status` != `Ok`"]
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct DirectionsTableOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[serde(rename = "errorMessage")]
    #[doc = "error message when `status` != `Ok`"]
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
    #[doc = "encoded geometry value in `polyline` or `polyline6`.\n\nFormat: [Link: Polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"]
    pub geometry: String,
    #[doc = "trip driving distance.\n\nUnit: `meters`"]
    pub distance: f64,
    #[doc = "special geospatial objects crossed along the trip."]
    pub special_objects: Option<HashMap<String, Vec<SpecialObject>>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct Route {
    #[doc = "encoded geometry value in `polyline` or `polyline6`.\n\nFormat: [Link: Polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"]
    pub geometry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`Debug Only!` encoded geometry value in `polyline` or `polyline6`.\n\nNote: might contains `raw` geometry before filtering.\n\nFormat: [Link: Polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"]
    pub geometry_full: Option<String>,
    #[doc = "route driving distance.\n\nUnit: `meters`"]
    pub distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_full: Option<f64>,
    #[doc = "route driving duration.\n\nUnit: `seconds`"]
    pub duration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "start location of route"]
    pub start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "end location of route"]
    pub end_location: Option<Location>,
    #[doc = "legs of route.\n\nNote: `waypoints` split `route` into `legs`"]
    pub legs: Option<Vec<Leg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "route driving duration before adjusting.\n\nNote: debug only."]
    pub raw_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "route driving duration after adjusting.\n\nNote: debug only."]
    pub predicted_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "special geospatial objects crossed along the trip."]
    pub special_objects: Option<HashMap<String, Vec<SpecialObject>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Apiv2Schema)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct Leg {
    #[doc = "leg driving distance.\n\nUnit: `meters`"]
    pub distance: IntValue,
    #[doc = "leg driving duration.\n\nUnit: `seconds`"]
    pub duration: IntValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "leg driving duration before adjusting.\n\nNote: debug only."]
    pub raw_duration: Option<IntValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "start location of `leg`"]
    pub start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "end location of `leg`"]
    pub end_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`steps` of `leg`"]
    pub steps: Option<Vec<Step>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct Step {
    #[doc = "encoded geometry value for step in `polyline` or `polyline6`.\n\nFormat: [Link: Polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"]
    pub geometry: Option<String>,
    #[doc = "start location of `step`"]
    pub start_location: Location,
    #[doc = "end location of `step`"]
    pub end_location: Location,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetNearbyInput {
    #[doc = "location of origin\n\nFormat: `lat,lng`\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
    pub currentlocation: String,
    #[doc = "mode of service.\n\nValues:`car|auto|bike|escooter|4w|2w...`.\n\nDefault: `\"\"`"]
    pub servicetype: String,
    #[doc = "radius to search.\n\nUnit: `meters`\n\nDefault: `10000`"]
    pub searchradius: Option<i64>,
    #[doc = "max number of `results`.\n\nDefault: `10`"]
    pub maxcount: Option<usize>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct GetNearbyOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[doc = "error message when `status` != `Ok`"]
    pub msg: Option<String>,
    #[doc = "location of origin"]
    pub currentLocation: Location,
    #[doc = "radius used to search.\n\nUnit: `meters`"]
    pub searchRadius: i64,
    #[doc = "max number of `results`."]
    pub maxCount: usize,
    #[doc = "service mode used.\n\nValues:`4w|3w|2w...`."]
    pub serviceType: String,
    pub results: Vec<NearbyResult>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct NearbyResult {
    pub id: String,
    #[doc = "result location."]
    pub location: Location,
    #[doc = "traveling duration to result location.\n\nUnit: `seconds`"]
    pub eta: u64,
    #[doc = "traveling distance to result location.\n\nUnit: `meters`"]
    pub distance: u64,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MatrixInput {
    #[doc = "locations of origins \n\nFormat: lat0,lng0|lat1,lng1|...\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$"]
    pub origins: String,
    #[doc = "locations of destinations\n\nFormat: lat0,lng0|lat1,lng1|...\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$"]
    pub destinations: String,
    #[doc = "mode of service.\n\nValues:`car|auto|bike|escooter|4w|2w...`.\n\nDefault: `\"\"`"]
    pub mode: Option<String>,
    #[doc = "departure time.\n\nFormat: `unix timestamp`.\n\nUnit: `seconds`.\n\nDefault: `0`"]
    pub departure_time: Option<i64>,
    #[doc = "enable to show debug information.\n\nDefault: `false`"]
    pub debug: Option<bool>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
    #[doc = "`deprecated`"]
    pub context: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MatrixOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[doc = "matrix output.\n\nNote: each row in following format\n\nRow[i]: `Element`(o[i]d[0]),`Element`(o[i]d[1]),`Element`(o[i]d[2])..."]
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Row {
    #[doc = "`elements` for a particular row|origin"]
    pub elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Element {
    #[doc = "traveling duration between origin and destination.\n\nUnit: `seconds`"]
    pub duration: IntValue,
    #[doc = "traveling distance between origin and destination.\n\nUnit: `seconds`"]
    pub distance: IntValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "traveling duration before adjust.\n\nUnit: `seconds`\n\nNote: debug only"]
    pub raw_duration: Option<IntValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "traveling duration after adjust.\n\nUnit: `seconds`\n\nNote: debug only"]
    pub predicted_duration: Option<IntValue>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct SnapInput {
    #[doc = "`locations` to perform `snap2roads`\n\nFormat: `lat0,lng0|lat1,lng1|...`\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$"]
    pub path: String,
    #[doc = "(unix timestamps for each `location`.\n\nUnit: `seconds`\n\nFormat: ts0|ts1|...\n\nRegex: ^[\\d]+(\\|[\\d]+)*$"]
    pub timestamps: Option<String>,
    #[doc = "radiuses of each `location` for performing `snap2road`\n\nUnit: `meters`\n\nFormat: `radius0|radius1|...`\n\nRegex: ^[\\d]+(\\|[\\d]+)*$"]
    pub radiuses: Option<String>,
    #[doc = "enable to interpolate the path.\n\nNote: might return more points\n\nDefault: `false`"]
    pub interpolate: Option<bool>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
    #[doc = "`deprecated`"]
    pub context: Option<String>,
    #[doc = "enable to ignore location not found in service boundary.\n\nNote: enable this to ignore outliers, otherwise an error will be thrown.\n\nDefault: `false`"]
    pub tolerate_outlier: Option<bool>,
}

#[derive(Serialize, Deserialize, Apiv2Schema, Debug)]
pub struct SnapOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[serde(rename = "snappedPoints")]
    pub snapped_points: Vec<SnappedPoint>,
    #[doc = "total travel distance of the snapped path\n\nUnit: `meters`"]
    pub distance: u64,
    #[doc = "encoded geometry value in `polyline` or `polyline6`.\n\nFormat: [Link: Polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"]
    pub geometry: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Apiv2Schema, Debug)]
pub struct SnappedPoint {
    pub location: Location,
    #[serde(rename = "originalIndex")]
    #[doc = "index of original input array"]
    pub original_index: u64,
    #[doc = "distance of the snapped point from the original\n\nUnit: `meters`"]
    pub distance: f64,
    #[doc = "name of the street the coordinate snapped to"]
    pub name: String,
    #[doc = "bearing angle of the snapped point.\n\nUnit: `radian`"]
    pub bearing: f64,
}
