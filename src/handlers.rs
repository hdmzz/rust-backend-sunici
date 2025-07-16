use actix_web::{post, web, HttpResponse, Responder};
use crate::models::{TerrainRequest};
use crate::rgb_model::{self};

#[post("/api/terrain_rgb")]
pub async fn get_terrain_rgb(params: web::Json<TerrainRequest>) -> impl Responder {
    let params_terrain: TerrainRequest = params.into_inner();

    let _result_data = rgb_model::get_terrain(&params_terrain);

    HttpResponse::Ok().body( format!( "Received Parameters for lat lon : {}, {}", params_terrain.lat, params_terrain.lon ))
}
