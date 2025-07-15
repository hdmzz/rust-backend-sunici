use serde::Deserialize;
use geo::{Destination, Haversine, Point};
use tilecover::tiles;
use crate::models::{BoundingBox};

// Please choose the correct path based on your project structure.

#[derive(Deserialize, Debug)]
pub struct TerrainRgbParams {
    pub lat: f64,
    pub lon: f64,
    pub radius: u32,
    pub zoom: u8,
    pub units_side: u32,
}

pub fn calculate_bbox(lat: f64, lon: f64, radius_meters: f64) -> BoundingBox {
    let center = Point::new(lon, lat);

    let nw_point = Haversine.destination(center, 315.0, radius_meters);

    let se_point = Haversine.destination(center, 135.0, radius_meters);

    BoundingBox {
        north_west: [nw_point.x(), nw_point.y()],
        south_east: [se_point.x(), se_point.y()],
    }
}

//http://localhost:8080/api/terrain_rgb?lat=45.7716711&lon=4.8376036&radius=5&zoom=15&units_side=10000
pub fn get_terrain(params: &TerrainRgbParams) {
    println!("lat {} , lon {}, radius {}, zoom {}, unistside {}", params.lat, params.lon, params.radius, params.zoom, params.units_side);

    let _units_per_meters: f64 = get_unit_per_meter(params.units_side, params.radius);

}

fn get_unit_per_meter(units_side: u32, radius: u32) -> f64 {
    let units_side_f: f64 = units_side as f64;
    let radius_f: f64 = radius as f64;

    units_side_f / ( radius_f * 2.0_f64.sqrt()  * 1000.0 )
}

