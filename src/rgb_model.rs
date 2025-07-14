use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TerrainRgbParams {
    pub lat: f64,
    pub lon: f64,
    pub radius: u32,
    pub zoom: u32,
    pub units_side: u32,
}


//http://localhost:8080/api/terrain_rgb?lat=45.7716711&lon=4.8376036&radius=5&zoom=15&units_side=10000
pub fn get_terrain(params: &TerrainRgbParams) {
    println!("lat {} , lon {}, radius {}, zoom {}, unistside {}", params.lat, params.lon, params.radius, params.zoom, params.units_side);
}
