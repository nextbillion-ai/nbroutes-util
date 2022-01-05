use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct LookupInput {
    #[doc = "Example: id=here:pds:place:276u33db-8097f3194e4b411081b761ea9a366776 
    Location ID, which is the ID of a result item eg. of a Discover request"]
    pub id: String,
    #[doc = "Select the language to be used for result rendering from a list of BCP 47 compliant language codes."]
    pub lang: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Address {
    pub label: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "countryName")]
    pub country_name: String,
    #[serde(rename = "stateCode")]
    pub state_code: String,
    pub state: String,
    #[serde(rename = "countyCode")]
    pub county_code: String,
    pub county: String,
    pub city: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub primary: bool,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct FoodType {}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Contact {
    pub phone: Vec<Value>,
    pub www: Vec<Value>,
    pub email: Vec<Value>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Chain {}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Value {
    pub value: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct OpeningHour {}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct LookupOutput {
    #[doc = "the type of result, e.g. place, address"]
    #[serde(rename = "resultType")]
    pub result_type: String,
    #[doc = "a representative string for the result, for instance the name of a place."]
    pub title: String,
    #[doc = "the detailed address of the result"]
    pub address: Address,
    #[doc = "a representative geo-position (WGS 84) of the result. this is to be used to display the result on a map"]
    pub position: Position,
    #[doc = "the geo-position of the access to the result (for instance the entrance)"]
    pub access: Vec<Position>,
    #[doc = "the identifier of the result object. Its value can be used to retrieve the very same object through the /lookup endpoint."]
    pub id: String,
    #[doc = "a list of category ids for place results.\n\n\
    The primary category has its flag primary set to true."]
    pub categories: Vec<Category>,
    #[doc = "a list of food-type ids for place results preparing/serving food.\n\n\
    The primary category has its flag primary set to true."]
    #[serde(rename = "foodTypes")]
    pub food_types: Option<Vec<FoodType>>,
    #[doc = "a list of chain ids for place results belonging to a chain.\n\n"]
    pub chains: Option<Vec<Chain>>,
    #[doc = "a list of contact details (phone, web, ...) for place results.\n\n"]
    pub contacts: Vec<Contact>,
    #[doc = "a list of opening hours for place results.\n\n"]
    #[serde(rename = "openingHours")]
    pub opening_hours: Vec<OpeningHour>,
}
