use crate::Result;
use geo::algorithm::contains::Contains;
use geo::{Point, Polygon};
use std::collections::HashMap;
pub struct Coord {
    lat: f64,
    lng: f64,
}

impl Coord {
    pub fn coord(input: &str) -> Result<Coord> {
        let items: Vec<&str> = input.split(",").collect();
        match items.len() {
            2 => Ok(Coord {
                lat: items[0].parse::<f64>()?,
                lng: items[1].parse::<f64>()?,
            }),
            _ => bail!("need 2 float for coordinate"),
        }
    }

    pub fn coords(input: &str) -> Result<Vec<Coord>> {
        let mut r: Vec<Coord> = Vec::new();
        let items = input.split("|");
        for item in items {
            r.push(Coord::coord(item)?);
        }
        Ok(r)
    }
}

impl Locatable for Coord {
    fn lat(&self) -> f64 {
        self.lat
    }
    fn lng(&self) -> f64 {
        self.lng
    }
}

pub trait Locatable {
    fn lat(&self) -> f64;
    fn lng(&self) -> f64;
    fn locate<'a>(&self, areas: &'a HashMap<String, Vec<Polygon<f64>>>) -> Result<&'a String> {
        let p = Point::<f64>::new(self.lng(), self.lat());
        for (k, vs) in areas.iter() {
            for v in vs {
                if v.contains(&p) {
                    return Ok(k);
                }
            }
        }
        bail!(format!("area not found for {},{}", self.lat(), self.lng()))
    }
}
