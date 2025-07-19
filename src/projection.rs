use crate::models::Bbox;
use std::f64::consts::PI;

/// Projette des coordonnées géographiques (lon, lat) dans un système de coordonnées
/// de scène 3D local défini par une Bbox et une taille de côté.
/// C'est l'équivalent de `HugoGeo.projectCoord`.
pub fn project_coords(lon_lat: [f64; 2], bbox: &Bbox, units_side: f64) -> [f64; 2] {
    let lon: f64 = lon_lat[0];
    let lat: f64 = lon_lat[1];

    let lon_delta: f64 = bbox.south_east[0] - bbox.north_west[0];
    let lat_delta: f64 = bbox.north_west[1] - bbox.south_east[1];

    let x_ratio: f64 = (lon - bbox.north_west[0]) / lon_delta;
    let y_ratio: f64 = (bbox.north_west[1] - lat) / lat_delta;

    let x: f64 = x_ratio * units_side - units_side / 2.0;
    let y: f64 = y_ratio * units_side - units_side / 2.0;

    [x, y]
}


pub fn spherical_mercator_ll(px: (f64, f64), zoom: u8, tile_size: f64) -> (f64, f64) {
    const R2D: f64 = 180.0 / PI;

    let size = tile_size * (2u32.pow(zoom as u32)) as f64;
    let bc = size / 360.0;
    let cc = size / (2.0 * PI);
    let zc = size / 2.0;
    let g = (px.1 - zc) / -cc;
    let lon = (px.0 - zc) / bc;
    let lat = R2D * (2.0 * (g.exp()).atan() - 0.5 * PI);

    (lon, lat)
}
