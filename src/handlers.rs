use actix_web::{post, web, HttpResponse, Responder, http::header};
use crate::models::{QuantizedTileData, TerrainRequest};
use crate::rgb_model::{RgbModel};
use byteorder::{LittleEndian, WriteBytesExt};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

#[post("/api/terrain_rgb")]
pub async fn get_terrain_rgb(params: web::Json<TerrainRequest>) -> impl Responder {
    let params_terrain: TerrainRequest = params.into_inner();

    let mut rgb_model = RgbModel {
        mapbox_token: "pk.eyJ1IjoiYWxhbnRnZW8tcHJlc2FsZXMiLCJhIjoiY2pzcTA4NjRiMTMxczQzcDFqa29maXk3bSJ9.pVYNTFKfcOXA_U_5TUwDWw".to_string(),
        api_rgb: "https://api.mapbox.com/v4/mapbox.terrain-rgb/".to_string(),
        data_elevation_covered: Vec::new(),
    };

    let result_data: Vec<(Vec<i32>, Vec<f64>, Vec<u32>)> = rgb_model.get_terrain(&params_terrain).await;

    if let Some((zoom_pos, vertices_f64, parent_tile)) = result_data.into_iter().next() {
        //Quantification
        let mut min_x = f64::MAX; let mut max_x = f64::MIN;
        let mut min_y = f64::MAX; let mut max_y = f64::MIN;
        let mut min_z = f64::MAX; let mut max_z = f64::MIN;

        for i in (0..vertices_f64.len()).step_by(3) {
            min_x = min_x.min(vertices_f64[i]); max_x = max_x.max(vertices_f64[i]);
            min_y = min_y.min(vertices_f64[i + 1]); max_y = max_y.max(vertices_f64[i + 1]);
            min_z = min_z.min(vertices_f64[i + 2]); max_z = max_z.max(vertices_f64[i +  2]);
        }

        let scale_x = if max_x == min_x { 0.0 } else { (u16::MAX as f64) / (max_x - min_x) };
        let scale_y = if max_y == min_y { 0.0 } else { (u16::MAX as f64) / (max_y - min_y) };
        let scale_z = if max_z == min_z { 0.0 } else { (u16::MAX as f64) / (max_z - min_z) };

        let mut quantized_vertices_u16 = Vec::with_capacity(vertices_f64.len());
        for i in (0..vertices_f64.len()).step_by(3) {
            quantized_vertices_u16.push(((vertices_f64[i]   - min_x) * scale_x) as u16);
            quantized_vertices_u16.push(((vertices_f64[i+1] - min_y) * scale_y) as u16);
            quantized_vertices_u16.push(((vertices_f64[i+2] - min_z) * scale_z) as u16);
        }

        let mut vertices_bytes = Vec::with_capacity(quantized_vertices_u16.len() * 2);
        for &val in &quantized_vertices_u16 {
            vertices_bytes.write_u16::<LittleEndian>(val).unwrap();
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&vertices_bytes).unwrap();
        let compressed_vertices = encoder.finish().unwrap();

        let metadata: QuantizedTileData = QuantizedTileData {
            zoom_pos, parent_tile,
            min_x, min_y, min_z,
            scale_x, scale_y, scale_z,
        };

        HttpResponse::Ok()
        .insert_header(("X-Tile-Metadata", serde_json::to_string(&metadata).unwrap()))
        .insert_header((header::CONTENT_TYPE, "application/octet-stream"))
        .body(compressed_vertices)
        
    } else {
        HttpResponse::BadRequest().finish()
    }

}
