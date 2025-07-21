use std::collections::HashSet;
use bytes::Bytes;
use ndarray::Array3;
use anyhow::Result;

use crate::{models::Bbox, projection::{project_coords, SphericalMercator}};

const CONST_VERTICES: usize = 128;

//handler qui va recevoir un zoom8position-> construire l'url, fetch trasnformation de donnees ==> addTile puis compisition du tableau
//[zoomPos, array dataElev, parentzoomPosition]
const MAPBOX_TOKEN: &str = "pk.eyJ1IjoiYWxhbnRnZW8tcHJlc2FsZXMiLCJhIjoiY2pzcTA4NjRiMTMxczQzcDFqa29maXk3bSJ9.pVYNTFKfcOXA_U_5TUwDWw";

pub fn get_url(zoom_position: &[u32]) -> String {
    let [z, x, y] = zoom_position else {
        panic!("zoom_pos doit contenir exactement trois éléments : [z, x, y]");
    };

    format!( "https://api.mapbox.com/v4/mapbox.terrain-rgb/{}/{}/{}@2x.pngraw?access_token={}", z, x, y, MAPBOX_TOKEN)
}

pub async fn get_pixels(url: &str) -> Result<Array3<u8>> {
    let bytes: bytes::Bytes = reqwest::get(url).await?.bytes().await?;

    let img: image::DynamicImage = image::load_from_memory(&bytes)?;

    let rgba_img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    let raw_pixel: Vec<u8> = rgba_img.into_raw();
    let pixel_array: ndarray::ArrayBase<ndarray::OwnedRepr<u8>, ndarray::Dim<[usize; 3]>> = Array3::from_shape_vec((height as usize, width as usize, 4), raw_pixel)?;

    Ok(pixel_array)
}

fn get_sixteenth_tiles_coords(zoom_position_elevation: &Vec<u32>) -> Vec<String> {
    let parent_z = zoom_position_elevation[0];
    let parent_x = zoom_position_elevation[1];
    let parent_y = zoom_position_elevation[2];

    let mut sixteenth: Vec<String> = Vec::with_capacity(16);
    for col in 0..4 {
        for row in 0..4 {
            let child_z = parent_z + 2;
            let child_x = parent_x * 4 + col;
            let child_y = parent_y * 4 + row;

            sixteenth.push(format!("{}/{}/{}", child_z, child_x, child_y));
        };
    };
    sixteenth        
}


pub fn add_tile_v2(tile_bytes: &Array3<u8>, zoom_position_elevation: &Vec<u32>, zp_covered: &[Vec<u32>], bbox: &Bbox, units_per_meter: f64) -> Vec<(Vec<f64>, Vec<f64>, Vec<u32>)> {
    let mercator = SphericalMercator::new(128.0);
    let elevations: Vec<f64> = tile_bytes
    .outer_iter()
    .map(|row: ndarray::ArrayBase<ndarray::ViewRepr<&u8>, ndarray::Dim<[usize; 2]>>| row.outer_iter().map(|pixel| {
        let r: f64 = pixel[0] as f64;
        let g: f64 = pixel[1] as f64;
        let b: f64 = pixel[2] as f64;

        -10000.0 + ((r * 256.0 * 256.0 + g * 256.0 + b) * 0.1)
    }).collect::<Vec<f64>>())
    .flatten()
    .collect();

    //Le tableau elevation contient les données d'elevation en mètres, pas de coordonnées donc il s'agit mainteant d'associer les données aux coordonnées

    let sixteenth: Vec<String> = get_sixteenth_tiles_coords(zoom_position_elevation);//a verifier
    let zoom_position_covered_str_set: HashSet<String> = zp_covered.iter().map(|zp| {format!("{}/{}/{}", zp[0], zp[1], zp[2])}).collect();
    let sixteenth_pixel_range: Vec<[[usize; 2]; 2]> = get_sixteenth_pixel_ranges();

    let mut data_elevation:  Vec<(Vec<f64>, Vec<f64>, Vec<u32>)> = Vec::new();

    for (index, zoom_pos_str) in sixteenth.iter().enumerate() {
        if !zoom_position_covered_str_set.contains(zoom_pos_str) {
            continue;
        };

        let zoom_pos : Vec<f64> = zoom_pos_str.split('/').map(|s| s.parse().expect("erreur")).collect();

        let px_range: &[[usize; 2]; 2] = &sixteenth_pixel_range[index];
        let mut elev: Vec<f64> = Vec::with_capacity(128 * 128);
        for r in px_range[0][0]..px_range[0][1] {
            for c in px_range[1][0]..px_range[1][1] {
                elev.push(elevations[r * 512 + c]);
            };
        };
        
        let mut array: Vec<f64> = Vec::with_capacity(CONST_VERTICES * CONST_VERTICES * 3);
        let mut data_index: usize = 0;
        
        for r in 0..128 {
            for c in 0..128 {
                let lon_lat_pixel = mercator.ll(
                    (zoom_pos[1] * 128.0 + c as f64, zoom_pos[2] * 128.0 + r as f64),
                    zoom_pos[0] as u8
                );

                let [x, y] = project_coords(
                    [lon_lat_pixel.0, lon_lat_pixel.1],
                    bbox,
                    10000.0
                );

                array.push(x);
                array.push(y);
                array.push(elev[data_index] * units_per_meter);
                data_index += 1;
            }
        }

        data_elevation.push((zoom_pos, array, zoom_position_elevation.clone()));
    };
        
    data_elevation
}

fn get_sixteenth_pixel_ranges() -> Vec<[[usize; 2]; 2]> {
    const COLS: usize = 512;
    const ROWS: usize = 512;
    const SCALE_FACTOR: usize = 4;
    const SUB_TILE_COUNT: usize = SCALE_FACTOR * SCALE_FACTOR; // 16

    let mut ranges = Vec::with_capacity(SUB_TILE_COUNT);

    let row_step = ROWS / SCALE_FACTOR; // 128
    let col_step = COLS / SCALE_FACTOR; // 128

    for c in 0..SCALE_FACTOR {
        for r in 0..SCALE_FACTOR {
            let row_start = r * row_step;
            let row_end = (r + 1) * row_step;

            let col_start = c * col_step;
            let col_end = (c + 1) * col_step;

            ranges.push([
                [row_start, row_end],
                [col_start, col_end],
            ]);
        }
    }

    ranges
}
