#![allow(non_snake_case)]
use crate::util::straight_distance;
use geo::{LineString, Polygon};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const STATUS_OK: &str = "Ok";
pub const STATUS_FAILED: &str = "Failed";

#[derive(Serialize, Deserialize, Clone, Apiv2Schema, PartialEq)]
pub enum GeometryInput {
    #[serde(rename = "polyline")]
    Polyline,
    #[serde(rename = "polyline6")]
    Polyline6,
    #[serde(rename = "geojson")]
    GeoJSON,
}

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub enum OverviewInput {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "simplified")]
    Simplified,
    #[serde(rename = "false")]
    False,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Geojson {
    pub coordinates: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Locations {
    pub id: u64,
    pub description: Option<String>,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Job {
    pub id: u64,
    pub location_index: i32,
    pub service: Option<u64>,
    pub delivery: Option<Vec<u64>>,
    pub pickup: Option<Vec<u64>>,
    pub time_windows: Option<Vec<Vec<f64>>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Vehicle {
    pub id: u64,
    pub start_index: Option<u64>,
    pub end_index: Option<u64>,
    pub capacity: Option<Vec<i64>>,
    pub time_window: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct VRoomResult {
    pub code: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub summary: Option<Summary>,
    pub unassigned: Option<Vec<Unassigned>>,
    pub routes: Option<Vec<VRoomRoute>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Unassigned {
    pub id: u64,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub location: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct VRoomRoute {
    pub vehicle: Option<u64>,
    pub cost: Option<u64>,
    pub steps: Option<Vec<VRoomStep>>,
    pub setup: Option<u64>,
    pub service: Option<u64>,
    pub duration: Option<f64>,
    pub waiting_time: Option<u64>,
    pub priority: Option<u64>,
    pub violations: Option<Vec<Violation>>,
    pub delivery: Option<u64>,
    pub pickup: Option<u64>,
    pub description: Option<String>,
    pub geometry: Option<String>,
    pub distance: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct VRoomStep {
    #[serde(rename = "type")]
    pub step_type: Option<String>,
    pub arrival: Option<f64>,
    pub duration: Option<f64>,
    pub setup: Option<u64>,
    pub service: Option<u64>,
    pub waiting_time: Option<u64>,
    pub violations: Option<Vec<Violation>>,
    pub description: Option<String>,
    pub location: Option<Vec<f64>>,
    pub id: Option<u64>,
    pub load: Option<f64>,
    pub distance: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Summary {
    pub cost: Option<u64>,
    pub routes: Option<u64>,
    pub unassigned: Option<u64>,
    pub setup: Option<u64>,
    pub service: Option<u64>,
    pub duration: Option<f64>,
    pub waiting_time: Option<u64>,
    pub priority: Option<u64>,
    pub violations: Option<Vec<Violation>>,
    pub delivery: Option<u64>,
    pub pickup: Option<u64>,
    pub distance: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Violation {
    pub cause: Option<String>,
    pub duration: Option<f64>,
}

// wrapper type to keep consistent with python api
#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct IntValue {
    pub value: u64,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct KeyInput {
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct UpdateRRTSimpleInput {
    pub from_way_id: u64,
    pub from_way_nodes: String,
    pub via_node_id: u64,
    pub via_node: String,
    pub to_way_id: u64,
    pub to_way_nodes: String,
    pub status: i32,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct UpdateRRTSimpleOutput {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct OptimizationInput {
    #[doc = "A semicolon-separated list of {lat},{lng}.\n\nFormat: `lat0,lng0|lat1,lng1|...`.\n\nRegex: (^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$)"]
    pub coordinates: String,
    #[doc = "mode of service.\n\nValues:`car`.\n\nDefault: `\"car\"`"]
    pub mode: Option<String>,
    #[doc = "The coordinate at which to start the returned route.\n\nValues: `any|first`.\n\nDefault: `first`"]
    pub source: Option<String>,
    #[doc = "Specify the destination coordinate of the returned route.\n\nValues: `any|last`.\n\nDefault: `any`"]
    pub destination: Option<String>,
    #[doc = "Indicates whether the returned route is roundtrip.\n\nDefault: `true`"]
    pub roundtrip: Option<bool>,
    #[doc = "Indicates whether the return geometry.\n\nDefault: `false`"]
    pub with_geometry: Option<bool>,
    #[doc = "output format of geometry.\n\nValue: `geojson|polyline|polyline6`.\n\nDefault: `polyline6`"]
    pub geometries: Option<String>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
    pub approaches: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct NavigatingInput {
    #[doc = "geometry input, if this is given, other params will not be considered except `geometry_type` & `lang` & `key`."]
    pub geometry: Option<String>,
    #[doc = "format of geometry.\n\nDefault: `polyline6`"]
    pub geometry_type: Option<String>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
    #[doc = "{{location_of_origin}}\n\nFormat: `lat,lng`.\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
    pub origin: Option<String>,
    #[doc = "location of destination.\n\nFormat: `lat,lng`.\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
    pub destination: Option<String>,
    #[doc = "location(s) of waypoint(s) along the trip.\n\nFormat: `lat0,lng0|lat1,lng1|...`.\n\nRegex: (^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$)"]
    pub waypoints: Option<String>,
    #[doc = "mode of service.\n\nValues:`car|auto|bike|escooter|4w|2w...`.\n\nDefault: `\"\"`"]
    pub mode: Option<String>,
    #[doc = "departure time.\n\nFormat: `unix timestamp`.\n\nUnit: `seconds`.\n\nDefault: `0`"]
    pub departure_time: Option<i64>,
    #[doc = "unique session id for trip identification.\n\nNote: Help to reuse cached trip characteritics when set. \n\nDefault: `\"\"`"]
    pub session: Option<String>,
    #[doc = "output verbosity of overview (whole trip) geometry.\n\nDefault: `full`"]
    pub overview: Option<OverviewInput>,
    #[doc = "number of alternative routes to return.\n\nDefault: `1` if `alternatives` is disabled, `3` otherwise"]
    pub altcount: Option<i32>,
    #[doc = "enable to return alternative routes.\n\nNote: `altcount` will default to `3` if this is enabled.\n\nDefault: `false`"]
    pub alternatives: Option<bool>,
    #[doc = "Indicates that the calculated route(s) should avoid the indicated features. \n\nFormat: `value1|value2|...`. Default:`\"\"`"]
    pub avoid: Option<String>,
    #[doc = "language of the text instruction"]
    pub lang: Option<String>,
    pub approaches: Option<String>,
    #[doc = "Limits the search to segments with given bearing in degrees towards true north in clockwise direction. \n\nFormat: `degree,range;degree,range...`. Default:`\"\"`"]
    pub bearings: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct NavigatingOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[doc = "`routes` calculated."]
    pub routes: Vec<Route>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "error message when `status` != `Ok`"]
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct ValhallaDirectionsInput {
    #[doc = "{{location_of_origin}}\n\nFormat: `lat,lng`.\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
    pub origin: String,
    #[doc = "location of destination.\n\nFormat: `lat,lng`.\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
    pub destination: String,
    #[doc = "location(s) of waypoint(s) along the trip.\n\nFormat: `lat0,lng0|lat1,lng1|...`.\n\nRegex: (^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$)"]
    pub waypoints: Option<String>,
    #[doc = "enable to include `steps` in response.\n\nDefault: `false`"]
    pub steps: Option<bool>,
    #[doc = "mode of service.\n\nValues:`car|auto|bike|escooter|4w|2w...`.\n\nDefault: `\"\"`"]
    pub mode: Option<String>,
    #[doc = "departure time, conflict with arrive_time.\n\nFormat: `unix timestamp`.\n\nUnit: `seconds`.\n\nDefault: `0`"]
    pub departure_time: Option<i64>,
    #[doc = "arrive time, conflict with departure_time.\n\nFormat: `unix timestamp`.\n\nUnit: `seconds`.\n\nDefault: `0`"]
    pub arrive_time: Option<i64>,
    #[doc = "unique session id for trip identification.\n\nNote: Help to reuse cached trip characteritics when set. \n\nDefault: `\"\"`"]
    pub session: Option<String>,
    #[doc = "output format of geometry.\n\nDefault: `polyline`"]
    pub geometry: Option<GeometryInput>,
    #[doc = "output verbosity of overview (whole trip) geometry.\n\nDefault: `full`"]
    pub overview: Option<OverviewInput>,
    #[doc = "number of alternative routes to return.\n\nDefault: `1` if `alternatives` is disabled, `3` otherwise"]
    pub altcount: Option<i32>,
    #[doc = "enable to return alternative routes.\n\nNote: `altcount` will default to `3` if this is enabled.\n\nDefault: `false`"]
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
    #[doc = "Indicates that the calculated route(s) should avoid the indicated features. \n\nFormat: `value1|value2|...`. Default:`\"\"`"]
    pub avoid: Option<String>,
    pub approaches: Option<String>,
    #[doc = "Indicates the truck size in CM, only valid when mode=6w. \n\nFormat: `height,width,length`."]
    pub truck_size: Option<String>,
    #[doc = "Indicates the truck weight including trailers and shipped goods in KG, only valid when mode=6w."]
    pub truck_weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct DirectionsInput {
    #[doc = "{{location_of_origin}}\n\nFormat: `lat,lng`.\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+$"]
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
    #[doc = "output format of geometry.\n\nDefault: `polyline`"]
    pub geometry: Option<GeometryInput>,
    #[doc = "output verbosity of overview (whole trip) geometry.\n\nDefault: `full`"]
    pub overview: Option<OverviewInput>,
    #[doc = "number of alternative routes to return.\n\nDefault: `1` if `alternatives` is disabled, `3` otherwise"]
    pub altcount: Option<i32>,
    #[doc = "enable to return alternative routes.\n\nNote: `altcount` will default to `3` if this is enabled.\n\nDefault: `false`"]
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
    #[doc = "Indicates that the calculated route(s) should avoid the indicated features. \n\nFormat: `value1|value2|...`. Default:`\"\"`"]
    pub avoid: Option<String>,
    pub approaches: Option<String>,
    #[doc = "Indicates the truck size in CM, only valid when mode=6w. \n\nFormat: `height,width,length`."]
    pub truck_size: Option<String>,
    #[doc = "Indicates the truck weight including trailers and shipped goods in KG, only valid when mode=6w."]
    pub truck_weight: Option<i32>,
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
pub struct OptimizationOutput {
    #[doc = "`Ok` for success."]
    pub code: String,
    #[doc = "Each waypoint is an input coordinate snapped to the road and path network."]
    pub waypoints: Vec<OptimizationWaypoint>,
    #[doc = "An array of 0 or 1 trip objects."]
    pub trips: Vec<OptimizationTrip>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct OptimizationPostInput {
    pub key: Option<String>,
    pub locations: Locations,
    pub jobs: Vec<Job>,
    pub vehicles: Vec<Vehicle>,
    pub mode: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct OptimizationPostOutput {
    pub id: String,
    pub message: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct OptimizationGetInput {
    pub key: Option<String>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct OptimizationGetOutput {
    pub result: VRoomResult,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct OptimizationWaypoint {
    pub name: String,
    pub location: Location,
    pub trips_index: i64,
    pub waypoint_index: i64,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct OptimizationTrip {
    pub geometry: String,
    pub legs: Vec<OptimizationLeg>,
    pub duration: f64,
    pub distance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geojson: Option<Geojson>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct OptimizationLeg {
    pub distance: f64,
    pub duration: f64,
    #[doc = "summary for this leg"]
    pub summary: String,
    pub steps: Vec<OptimizationStep>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema)]
pub struct OptimizationStep {
    pub distance: f64,
    pub duration: f64,
    pub geometry: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geojson: Option<Geojson>,
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

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geojson: Option<Geojson>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Apiv2Schema)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Annotation {
    pub duration: Vec<f64>,
    pub distance: Vec<f64>,
    pub speed: Vec<f64>,
    pub weight: Vec<f64>,
    pub nodes: Vec<i64>,
    pub datasources: Vec<i32>,
    pub metadata: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct MetaData {
    pub datasource_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`deprecated`"]
    pub annotation: Option<Annotation>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Step {
    #[doc = "encoded geometry value for step in `polyline` or `polyline6`.\n\nFormat: [Link: Polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)"]
    pub geometry: Option<String>,
    #[doc = "start location of `step`"]
    pub start_location: Location,
    #[doc = "end location of `step`"]
    pub end_location: Location,
    #[doc = "step driving distance.\n\nUnit: `meters`"]
    pub distance: IntValue,
    #[doc = "step driving duration.\n\nUnit: `seconds`"]
    pub duration: IntValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "step Maneuver"]
    pub maneuver: Option<Maneuver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "step name"]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "step intersections"]
    pub intersections: Option<Vec<Intersection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geojson: Option<Geojson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "step reference"]
    pub reference: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Intersection {
    pub location: Coordinate,
    pub bearings: Vec<i32>,
    pub classes: Vec<String>,
    pub entry: Vec<bool>,
    pub intersection_in: i32,
    pub intersection_out: i32,
    pub lanes: Vec<Lane>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Lane {
    pub indications: Vec<String>,
    pub valid: bool,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct VoiceInstruction {
    pub distance_along_geometry: i32,
    pub unit: String,
    pub instruction: String,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Maneuver {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    pub voice_instruction: Vec<VoiceInstruction>,
    pub bearing_before: i32,
    pub bearing_after: i32,
    pub coordinate: Coordinate,
    pub maneuver_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Apiv2Schema, Clone)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct ValhallaMatrixInput {
    #[doc = "locations of origins \n\nFormat: lat0,lng0|lat1,lng1|...\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$"]
    pub origins: String,
    #[doc = "locations of destinations\n\nFormat: lat0,lng0|lat1,lng1|...\n\nRegex: ^[\\d\\.\\-]+,[\\d\\.\\-]+(\\|[\\d\\.\\-]+,[\\d\\.\\-]+)*$"]
    pub destinations: String,
    #[doc = "mode of service.\n\nValues:`car|auto|bike|escooter|4w|2w...`.\n\nDefault: `\"\"`"]
    pub mode: Option<String>,
    #[doc = "departure time, conflict with arrive_time.\n\nFormat: `unix timestamp`.\n\nUnit: `seconds`.\n\nDefault: `0`"]
    pub departure_time: Option<i64>,
    #[doc = "arrive time, conflict with departure_time.\n\nFormat: `unix timestamp`.\n\nUnit: `seconds`.\n\nDefault: `0`"]
    pub arrive_time: Option<i64>,
    #[doc = "enable to show debug information.\n\nDefault: `false`"]
    pub debug: Option<bool>,
    #[doc = "apikey for authentication.\n\nDefault: `\"\"`"]
    pub key: Option<String>,
    #[doc = "`deprecated`"]
    pub context: Option<String>,
    pub avoid: Option<String>,
    pub approaches: Option<String>,
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
    pub avoid: Option<String>,
    pub approaches: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MatrixOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[doc = "matrix output.\n\nNote: each row in following format\n\nRow[i]: `Element`(o[i]d[0]),`Element`(o[i]d[1]),`Element`(o[i]d[2])..."]
    pub rows: Vec<Row>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MatrixConciseOutput {
    #[doc = "`Ok` for success."]
    pub status: String,
    #[doc = "matrix output.\n\n
|e00,d00|e01,d01|e02,d02...|\n
|e10,d10|e11,d11|e12,d02...|\n
|e20,d00|e21,d01|e22,d02...|\n
...\n
where:\n
e(xy) eta for origins[x] to dest[y]\n
d(xy) distance for origins[x] to dest[y]\n
"]
    pub rows: Vec<Vec<Vec<u64>>>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Row {
    #[doc = "`elements` for a particular row|origin"]
    pub elements: Vec<Element>,
}

#[derive(Serialize, Deserialize, Apiv2Schema, Clone)]
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
pub struct IsochroneInput {
    pub center: String,
    pub resolution: Option<i32>,
    pub times: Option<String>,
    pub distances: Option<String>,
    pub strokes: Option<String>,
    pub opacities: Option<String>, // range: [0, 1], 0 for transparent
    pub mode: Option<String>,
    pub departure_time: Option<i64>,
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct IsochroneOutput {
    pub status: String,
    pub polylines: Vec<String>,
    pub strokes: Option<Vec<String>>,
    pub opacities: Option<Vec<f64>>,
    pub times: Option<Vec<i32>>,
    pub distances: Option<Vec<i32>>,
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
    pub mode: Option<String>,
    pub avoid: Option<String>,
    pub approaches: Option<String>,
    #[doc = "only supports for polyline and geojson"]
    pub geometry: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geojson: Option<Vec<Option<Geojson>>>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigCoord {
    pub lat: f64,
    pub lng: f64,
}

impl ConfigCoord {
    pub fn distance(&self, someone: &ConfigCoord) -> f64 {
        straight_distance(self.lat, self.lng, someone.lat, someone.lng)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigPolygon {
    pub name: String,
    pub coords: Vec<ConfigCoord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigArea {
    pub id: String,
    pub polygons: Vec<ConfigPolygon>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigCluster {
    pub id: String,
    pub address: String,
    pub nbroutes: Vec<String>,
    pub location: ConfigCoord,
    //for example: singapore-4w: {matrix_size: {name: large, value: 10000}}
    //which is saying for singapore-4w sku, if matrix-size > 10000, feature=large
    pub features: Option<HashMap<String, HashMap<String, Vec<ConfigKeyValue>>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigKeyValue {
    pub name: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaaasAreaConfig {
    pub areas: Vec<ConfigArea>,
    #[serde(skip)]
    pub parsed_areas: HashMap<String, Vec<Polygon<f64>>>,
    #[serde(skip)]
    pub inited: bool,
}

impl MaaasAreaConfig {
    pub fn init(&mut self) {
        if self.inited {
            return;
        }
        for area in self.areas.iter() {
            let mut polygons: Vec<Polygon<f64>> = Vec::new();
            for p in area.polygons.iter() {
                let mut coords: Vec<(f64, f64)> = Vec::new();
                for c in p.coords.iter() {
                    coords.push((c.lng, c.lat));
                }
                polygons.push(Polygon::<f64>::new(LineString::from(coords), vec![]));
            }
            self.parsed_areas.insert(area.id.to_owned(), polygons);
        }
        self.inited = true;
    }

    pub fn polygons(&mut self, area: &str) -> Option<&Vec<Polygon<f64>>> {
        self.init();
        self.parsed_areas.get(area)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaaasConfig {
    pub clusters: Vec<ConfigCluster>,
}

#[derive(Debug)]
pub struct MaaasLookupResult {
    pub local: bool,
    pub proxy_address: Option<String>,
}

impl MaaasConfig {
    pub fn lookup(&self, cluster_id: &str, nbroute: &str) -> Option<MaaasLookupResult> {
        let mut self_cluster: Option<&ConfigCluster> = None;
        for cluster in self.clusters.iter() {
            if cluster.id == cluster_id {
                self_cluster = Some(&cluster);
                break;
            }
        }
        for r in self_cluster?.nbroutes.iter() {
            if r == nbroute {
                return Some(MaaasLookupResult {
                    local: true,
                    proxy_address: None,
                });
            }
        }
        let mut proxy_address: Option<&str> = None;
        let mut min_dist: f64 = -1.0;
        for cluster in self.clusters.iter() {
            for r in cluster.nbroutes.iter() {
                if r == nbroute {
                    let dist = self_cluster?.location.distance(&cluster.location);
                    if min_dist < 0.0 || min_dist > dist {
                        min_dist = dist;
                        proxy_address = Some(&cluster.address);
                    }
                }
            }
        }
        Some(MaaasLookupResult {
            local: false,
            proxy_address: Some(proxy_address?.to_owned()),
        })
    }
}

// KeySKUSetting is not needed now but leaves the room for things like rate limit etc
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeySKUSetting {
    pub sku_id: i64,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct KeyServerAuthKeyDecodedSource {
    pub referers: Option<Vec<String>>,
    pub origins: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyServerAuthKey {
    pub source: Option<KeyServerAuthKeyDecodedSource>,
    pub sku_map: Option<HashMap<String, KeySKUSetting>>,
    pub labels: Option<HashMap<String, String>>,
    pub qps_limit: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        {
            let content = "clusters:\n
  - id: aks-sg\n
    address: https://maaas-aks-sg.nextbillion.io\n
    nbroutes:\n
      - singapore-4w\n
      - india-4w\n
      - ca-4w\n
    location:\n
      lat: 1.3437459\n
      lng: 103.8240449\n
  - id: aks-ld\n
    address: https://maaas-aks-ld.nextbillion.io\n
    nbroutes: []\n
    location:\n
      lat: 51.5287352\n
      lng: -0.3817863";
            let r: MaaasConfig = serde_yaml::from_str(content).unwrap();
            {
                let lr = r.lookup("aks-sg", "singapore-4w");
                assert!(lr.is_some());
                let lr = lr.unwrap();
                assert!(lr.local);
            }
            {
                let lr = r.lookup("aks-sg", "singapore-8w");
                assert!(lr.is_none());
            }
            {
                let lr = r.lookup("aks-ld", "singapore-4w");
                assert!(lr.is_some());
                let lr = lr.unwrap();
                assert!(!lr.local);
                assert!(lr.proxy_address.is_some());
                assert!(lr.proxy_address.unwrap() == "https://maaas-aks-sg.nextbillion.io");
            }
        }
        {
            let content = "areas:\n
  - id: singapore\n
    polygons:\n
      - name: area1\n
        coords:\n
          - lng: 103.80844116210938\n
            lat: 1.4802430218865072\n
          - lng: 103.7164306640625\n
            lat: 1.4596504356431457\n
          - lng: 103.65875244140625\n
            lat: 1.4267019064882447\n
          - lng: 103.57498168945312\n
            lat: 1.2317471514699085\n
          - lng: 103.73428344726561\n
            lat: 1.139756366394449\n
          - lng: 104.0679931640625\n
            lat: 1.334718132769963\n
          - lng: 103.97872924804688\n
            lat: 1.4308204986633148\n
          - lng: 103.80844116210938\n
            lat: 1.4802430218865072\n";

            let mut r: MaaasAreaConfig = serde_yaml::from_str(content).unwrap();

            let pl = r.polygons("singapore");
            assert!(pl.is_some());
            let pl = pl.unwrap();
            assert!(pl.len() == 1);
            assert!(r.areas.len() == 1);
        }
    }
}
