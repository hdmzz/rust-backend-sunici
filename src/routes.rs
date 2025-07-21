use actix_web::web;

use crate::handlers::{
    get_tile
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tile);
}
