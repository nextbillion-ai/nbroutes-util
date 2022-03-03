use crate::util::Area;
use crate::Result;
use geo::algorithm::contains::Contains;
use geo::prelude::BoundingRect;
use geo::{Point, Polygon};
use std::collections::HashMap;

#[derive(Debug)]
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

    pub fn coords_to_str(input: Vec<&Coord>) -> String {
        let mut point_strs = vec![];
        for coord in input.iter() {
            let point_str = format!("{},{}", coord.lat, coord.lng);
            point_strs.push(point_str);
        }
        point_strs.join("|")
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
    fn locate<'a>(
        &self,
        area_polygons: &HashMap<String, Vec<Polygon<f64>>>,
        selected_areas: &'a Vec<Area>,
    ) -> Result<&'a Area> {
        let p = Point::<f64>::new(self.lng(), self.lat());
        for area in selected_areas.iter() {
            let vs = area_polygons.get(area.name.as_str());
            if vs.is_none() {
                warn!("area name {} doesn't have polylgon", area.name.as_str());
                continue;
            }

            for v in vs.unwrap() {
                if v.contains(&p) {
                    return Ok(area);
                }
            }
        }

        bail!(format!("area not found for {},{}", self.lat(), self.lng()))
    }

    fn is_in_polygons<'a>(&self, polygons: &Vec<Polygon<f64>>) -> bool {
        let p = Point::<f64>::new(self.lng(), self.lat());
        for v in polygons {
            let brect = v.bounding_rect().unwrap();
            if p.x() < brect.min().x
                || p.x() > brect.max().x
                || p.y() < brect.min().y
                || p.y() > brect.max().y
            {
                continue;
            }
            if v.contains(&p) {
                return true;
            }
        }

        return false;
    }
}
