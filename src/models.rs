use serde::{Deserialize, Serialize};
use core::fmt;
use std::fmt::{Display};

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {}

#[derive(Deserialize, Debug)]
pub struct PolygonFeature {
    #[serde(rename = "type")]
    pub _type: String,
    geometry : Geometry,
}

#[derive(Debug)]
pub struct ProcessedTileData {
    pub zoom_pos: [u32; 3],
    pub vertex_array: Vec<f64>, // [x1, y1, z1, x2, y2, z2, ...]
}

#[derive(Debug, Serialize)]
pub struct TerrainMesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub uvs: Vec<f32>,
    pub satellite_texture_url: String,
    pub zoom_pos: [u32; 3],
}

use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct TerrainRequest {
    pub lat: f64,
    pub lon: f64,
    pub radius: f64,
    pub zoom: u32,
    pub units_per_meter: f64,
    pub zoom_position_covered: Vec<Vec<u32>>,
    pub bbox: Bbox,
}

#[derive(Serialize)]
pub struct QuantizedTileData {
    pub zoom_pos: Vec<i32>,
    pub parent_tile: Vec<u32>,
    // Métadonnées pour la dé-quantification
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
    // Les sommets quantifiés et compressés seront dans le corps de la réponse binaire
}

#[derive(Serialize)]
pub struct TileResponseData {
   pub zoom_pos: Vec<i32>,
   pub parent_tile: Vec<u32>,
   pub vertices_base64: String,
} 

#[derive(Serialize, Deserialize, Debug)]
pub struct Bbox {
    pub feature: Feature,
    // L'attribut `rename` est utilisé pour mapper les noms camelCase de JavaScript
    // aux noms snake_case idiomatiques de Rust.
    #[serde(rename = "northWest")]
    pub north_west: [f64; 2],
    #[serde(rename = "southEast")]
    pub south_east: [f64; 2],
}

impl Display for Bbox {
    fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        match serde_json::to_string_pretty( self ) {
            Ok( json_string ) => write!( f, "{}", json_string ),
            Err( _ ) => write!( f, "Erreur lors de lla serialisation de la Bbox en JSON" ),
        }
    }
}

/// Représente l'objet `feature` dans `bbox`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    #[serde(rename = "type")]
    pub feature_type: String,
    pub geometry: Geometry,
}

/// Représente l'objet `geometry` dans `feature`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    // `properties` est de type `Value` pour plus de flexibilité,
    // car il est vide dans votre cas.
    pub properties: Value,
    #[serde(rename = "type")]
    pub geometry_type: String,
    // Les coordonnées d'un polygone GeoJSON.
    pub coordinates: Vec<Vec<[f64; 2]>>,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub data: Vec<u8>,
    pub offset: u8,
    pub shape: [u8; 3],
    pub stride: [u8; 3],
}
