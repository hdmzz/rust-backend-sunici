use std::env;

use actix_web::{post, web, HttpResponse, Responder};
use crate::models::{TerrainRequest};
use crate::rgb_model::{self, RgbModel};

#[post("/api/terrain_rgb")]
pub async fn get_terrain_rgb(params: web::Json<TerrainRequest>) -> impl Responder {
    let params_terrain: TerrainRequest = params.into_inner();

    let mut rgb_model = RgbModel {
        mapbox_token: "pk.eyJ1IjoiYWxhbnRnZW8tcHJlc2FsZXMiLCJhIjoiY2pzcTA4NjRiMTMxczQzcDFqa29maXk3bSJ9.pVYNTFKfcOXA_U_5TUwDWw".to_string(),
        api_rgb: "https://api.mapbox.com/v4/mapbox.terrain-rgb/".to_string(),
        data_elevation_covered: Vec::new(),
    };

    let _result_data = rgb_model.get_terrain(&params_terrain).await;

    HttpResponse::Ok().body( format!( "Received Parameters for lat lon : {}, {}", params_terrain.lat, params_terrain.lon ))
}
