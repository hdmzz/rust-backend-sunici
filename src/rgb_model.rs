//use actix_web::{error::ErrorInternalServerError, Error};
//use crate::{models::{Bbox, TerrainRequest}, projection::project_coords};
//use std::collections::{HashMap, HashSet};
//use futures::future::join_all;
//use bytes::{Bytes};
////use crate::projection::spherical_mercator_ll;

//const CONST_VERTICES: usize = 128;

//pub fn get_zoom_position_elevation(zp_covered: &[Vec<u32>]) -> Vec<Vec<u32>> {
//    let mut elevations: HashMap<String, Vec<Vec<u32>>> = HashMap::new();

//    for zoom_pos in zp_covered {
//        if zoom_pos.len() == 3 {
//            if let Some(z) = zoom_pos[0].checked_sub(2) {//verifie si le niveau de zoom n'est pas inferieur a 2 sinon return None
//                let grandparent_z = z;
//                let grandparent_x = zoom_pos[1] / 4;
//                let grandparent_y = zoom_pos[2] / 4;

//                let key = format!("{},{},{}", grandparent_z, grandparent_x, grandparent_y);

//                elevations.entry(key).or_default().push(zoom_pos.to_vec());
//            }
//        }
//    }

//    elevations
//        .keys()
//        .map(|triplet_str| {
//            triplet_str
//                .split(',')
//                .filter_map(|num_str| num_str.parse::<u32>().ok())
//                .collect::<Vec<u32>>()
//        })
//        .collect()
//}


//pub async fn fetch_data( url: &String ) -> Result<Bytes, Error> {
//    let response = reqwest::get(url)
//        .await
//        .map_err(ErrorInternalServerError)?;
//    let image_bytes = response
//    .bytes()
//    .await
//    .map_err(ErrorInternalServerError);

//    Ok(image_bytes?)
//}

//pub fn get_sixteenth_pixel_ranges() -> Vec<[[usize; 2]; 2]> {
//    const COLS: usize = 512;
//    const ROWS: usize = 512;
//    const SCALE_FACTOR: usize = 4;
//    const SUB_TILE_COUNT: usize = SCALE_FACTOR * SCALE_FACTOR; // 16

//    let mut ranges = Vec::with_capacity(SUB_TILE_COUNT);

//    let row_step = ROWS / SCALE_FACTOR; // 128
//    let col_step = COLS / SCALE_FACTOR; // 128

//    for c in 0..SCALE_FACTOR {
//        for r in 0..SCALE_FACTOR {
//            let row_start = r * row_step;
//            let row_end = (r + 1) * row_step;

//            let col_start = c * col_step;
//            let col_end = (c + 1) * col_step;

//            ranges.push([
//                [row_start, row_end],
//                [col_start, col_end],
//            ]);
//        }
//    }

//    ranges
//}

//pub struct RgbModel {
//    pub mapbox_token: String,
//    pub api_rgb: String,
//    pub data_elevation_covered: Vec<Vec<Vec<f64>>>,
//}

//impl RgbModel {

//    //http://localhost:8080/api/terrain_rgb?lat=45.7716711&lon=4.8376036&radius=5&zoom=15&units_side=10000
//    //point d'entrée du module
//    pub async fn get_terrain(&mut self, params: &TerrainRequest) -> Vec<(Vec<i32>, Vec<f64>, Vec<u32>)> {

//        //1 recuperer BBox
//        let bbox: &Bbox = &params.bbox;//la reference est utiliser si on ne lutilise pas cela creer une copie
//        //2 recuperer zoom position covered ==> les 81 tuiles .....
//        let zoom_position_covered: Vec<Vec<u32>> = params.zoom_position_covered
//        .clone();

//        let  ret: Vec<(Vec<i32>, Vec<f64>, Vec<u32>)> = self.fetch(&zoom_position_covered, bbox).await;

//        ret
//    }

//    pub async fn fetch( &mut self, zp_covered: &[Vec<u32>], bbox: &Bbox ) -> Vec<(Vec<i32>, Vec<f64>, Vec<u32>)> {
//        let zoom_position_elevation = get_zoom_position_elevation(zp_covered);

//        let fetch_futures = zoom_position_elevation
//        .iter()
//        .map(|zoom_position| {
//            let url =self.get_uri(&zoom_position).clone();
//            println!("{:?}", url);
//            async move {
//                fetch_data(&url).await
//            }
//        });

//        let tiles_results = join_all(fetch_futures).await;

//        let new_data_segments: Vec<(Vec<i32>, Vec<f64>, Vec<u32>)> = tiles_results
//        .into_iter()
//        .zip(zoom_position_elevation.iter())
//        .map(|(tile_result, zoom_pos)| {
//            let tile_bytes = tile_result.expect("Échec de la re cupération de la tuile");
//            self.add_tile(&tile_bytes, zoom_pos, zp_covered, bbox)
//        })
//        .flatten()
//        .collect::<Vec<_>>();

//        new_data_segments
//    }

//    fn add_tile(&self, tile_bytes: &Bytes, zoom_position_elevation: &Vec<u32>, zp_covered: &[Vec<u32>], bbox: &Bbox) -> Vec<(Vec<i32>, Vec<f64>, Vec<u32>)> {
//        let decoder: png::Decoder<&[u8]> = png::Decoder::new(tile_bytes.as_ref());
//        let mut reader: png::Reader<&[u8]> = match decoder.read_info() {
//            Ok(r) => r,
//            Err(_) => return Vec::new()
//        };

//        let mut buff: Vec<u8>= vec![0; reader.output_buffer_size()];

//        if reader.next_frame(&mut buff).is_err() {
//            return Vec::new();
//        }

//        let elevations: Vec<f64> = buff
//        .chunks_exact(4)
//        .map(|pixel: &[u8]| {
//            let r: f64 = pixel[0] as f64;
//            let g: f64 = pixel[1] as f64;
//            let b: f64 = pixel[2] as f64;

//            -10000.0 + ((r * 256.0 * 256.0 + g * 256.0 + b) * 0.1)
//        })
//        .collect();
//        //Le tableau elevation contient les données d'elevation en mètres, pas de coordonnées donc il s'agit mainteant d'associer les données aux coordonnées

//        let sixteenth: Vec<String> = self.get_sixteenth_tiles_coords(zoom_position_elevation);
//        let zoom_position_covered_str_set: HashSet<String> = zp_covered.iter().map(|zp| {format!("{}/{}/{}", zp[0], zp[1], zp[2])}).collect();
//        let sixteenth_pixel_range: Vec<[[usize; 2]; 2]> = get_sixteenth_pixel_ranges();

//        let mut data_elevation:  Vec<(Vec<i32>, Vec<f64>, Vec<u32>)> = Vec::new();

//        for (index, zoom_pos_str) in sixteenth.iter().enumerate() {
//            if !zoom_position_covered_str_set.contains(zoom_pos_str) {
//                continue;
//            };

//            let zoom_pos : Vec<i32> = zoom_pos_str.split('/').map(|s| s.parse().expect("erreur")).collect();

//            let px_range: &[[usize; 2]; 2] = &sixteenth_pixel_range[index];
//            let mut elev: Vec<f64> = Vec::with_capacity(128 *128);
//            for r in px_range[0][0]..px_range[0][1] {
//                for c in px_range[1][0]..px_range[1][1] {
//                    elev.push(elevations[r * 512 + c]);
//                };
//            };
            
//            let mut array: Vec<f64> = Vec::with_capacity(CONST_VERTICES * CONST_VERTICES * 3);
//            let mut data_index: usize = 0;
            
//            for r in 0..128 {
//                for c in 0..128 {
//                    let lon_lat_pixel = spherical_mercator_ll(
//                        ((zoom_pos[1] as usize * 128 + c) as f64,
//                        (zoom_pos[2] as usize * 128 + r) as f64),
//                        zoom_pos[0] as u8, 
//                        512.0
//                    );

//                    let [x, y] = project_coords(
//                        [lon_lat_pixel.0, lon_lat_pixel.1],
//                        bbox,
//                        10000.0
//                    );

//                    array.push(x);
//                    array.push(y);
//                    array.push(elevations[data_index] * 10000.0);
//                    data_index += 1;
//                }
//            }

//            data_elevation.push((zoom_pos, array, zoom_position_elevation.clone()));
//        };
            
//        data_elevation
//    }

//    fn get_uri(&self, zoom_pos: &[u32]) -> String {
//        let [z, x, y] = zoom_pos else {
//            panic!("zoom_pos doit contenir exactement trois éléments : [z, x, y]");
//        };
    
//        format!( "https://api.mapbox.com/v4/mapbox.terrain-rgb/{}/{}/{}@2x.pngraw?access_token={}", z, x, y, self.mapbox_token)
//    }

//    fn get_sixteenth_tiles_coords(&self, zoom_position_elevation: &Vec<u32>) -> Vec<String> {
//        let parent_z = zoom_position_elevation[0];
//        let parent_x = zoom_position_elevation[1];
//        let parent_y = zoom_position_elevation[2];

//        let mut sixteenth: Vec<String> = Vec::with_capacity(16);
//        for col in 0..4 {
//            for row in 0..4 {
//                let child_z = parent_z + 2;
//                let child_x = parent_x * 4 + col;
//                let child_y = parent_y * 4 + row;

//                sixteenth.push(format!("{}/{}/{}", child_z, child_x, child_y));
//            };
//        };
//        sixteenth        
//    }
//}
