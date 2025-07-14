use actix_web::web;

use crate::handlers::{
    get_terrain_rgb
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_terrain_rgb);
}
