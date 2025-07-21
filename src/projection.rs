use crate::models::Bbox;
use std::f64::consts::PI;

/// Projette des coordonnées géographiques (lon, lat) dans un système de coordonnées
/// de scène 3D local défini par une Bbox et une taille de côté.
/// C'est l'équivalent de `HugoGeo.projectCoord`.
pub fn project_coords(lon_lat: [f64; 2], bbox: &Bbox, units_side: f64) -> [f64; 2] {
    let lon: f64 = lon_lat[0];
    let lat: f64 = lon_lat[1];

    let x: f64 = units_side * (-0.5 + (lon - bbox.north_west[0]) / (bbox.south_east[0] - bbox.north_west[0]));
    let y: f64 = units_side * (-0.5 - (lat - bbox.south_east[1]) / (bbox.south_east[1] - bbox.north_west[1]));

    [x, y]
}

const MAX_ZOOM: usize = 30;


pub struct SphericalMercator {
    bc: Vec<f64>,
    cc: Vec<f64>,
    zc: Vec<f64>,
    ac: Vec<f64>,
}

impl SphericalMercator {
    pub fn new(tile_size: f64) -> Self {
        let mut bc = Vec::with_capacity(MAX_ZOOM);
        let mut cc = Vec::with_capacity(MAX_ZOOM);
        let mut zc = Vec::with_capacity(MAX_ZOOM);
        let mut ac = Vec::with_capacity(MAX_ZOOM);

        let mut size = tile_size;
        for _ in 0..MAX_ZOOM {
            bc.push(size / 360.0);
            cc.push(size / (2.0 * PI));
            zc.push(size / 2.0);
            ac.push(size);
            size *= 2.0;
        }

        SphericalMercator { bc, cc, zc, ac }
    }

    /// Convertit des coordonnées pixel (x, y) en coordonnées géographiques (longitude, latitude).
    pub fn ll(&self, px: (f64, f64), zoom: u8) -> (f64, f64) {
        const R2D: f64 = 180.0 / PI;
        let zoom_idx = zoom as usize;

        if zoom_idx >= MAX_ZOOM {
            // Ou gérez l'erreur comme vous le souhaitez
            panic!("Zoom level {} is out of supported range [0, {})", zoom, MAX_ZOOM);
        }

        let g = (px.1 - self.zc[zoom_idx]) / -self.cc[zoom_idx];
        let lon = (px.0 - self.zc[zoom_idx]) / self.bc[zoom_idx];
        let lat = R2D * (2.0 * (g.exp()).atan() - 0.5 * PI);

        (lon, lat)
    }
}
