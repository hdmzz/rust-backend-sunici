use actix_web::{error::ErrorInternalServerError, Error};
use crate::models::{ApiResponse, Bbox, TerrainRequest};
use std::{collections::HashMap, env};
use futures::future::join_all;
const CONST_VERTICES: usize = 128;

pub fn get_zoom_position_elevation(zp_covered: &[Vec<u32>]) -> Vec<Vec<u32>> {
    // HashMap pour stocker les groupements. La clé est une chaîne de caractères représentant
    // les coordonnées du grand-parent ("z,x,y").
    let mut elevations: HashMap<String, Vec<Vec<u32>>> = HashMap::new();

    for zoom_pos in zp_covered {
        // S'assure que la tuile a bien 3 composantes [z, x, y].
        if zoom_pos.len() == 3 {
            // Calcule les coordonnées du grand-parent.
            // `checked_sub` est utilisé pour éviter une panique si z < 2.
            if let Some(z) = zoom_pos[0].checked_sub(2) {//verifie si le niveau de zoom n'est pas inferieur a 2 sinon return None
                let grandparent_z = z;
                // La division entière sur des entiers positifs en Rust est équivalente à Math.floor().
                let grandparent_x = zoom_pos[1] / 4;
                let grandparent_y = zoom_pos[2] / 4;

                // Crée la clé pour le HashMap.
                let key = format!("{},{},{}", grandparent_z, grandparent_x, grandparent_y);

                // Utilise l'API `entry` pour insérer une nouvelle entrée ou ajouter à une existante.
                // `or_default()` insère un `Vec` vide si la clé n'existe pas, puis `push` ajoute l'élément qui est une copie==> to_vec()
                elevations.entry(key).or_default().push(zoom_pos.to_vec());
            }
        }
    }

    // La fonction originale renvoie les clés uniques (les grands-parents).
    // Nous extrayons les clés du HashMap, les divisons et les reconvertissons en nombres.
    elevations
        .keys()
        .map(|triplet_str| {
            triplet_str
                .split(',')
                // `filter_map` est utilisé pour parser chaque partie en u32.
                // Si le parsing échoue, `ok()` renvoie `None` et l'élément est ignoré.
                .filter_map(|num_str| num_str.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect()
}


async fn fetch_data( url: &String ) -> Result<ApiResponse, Error> {
    let response = reqwest::get(url)
        .await
        .map_err(ErrorInternalServerError)?;
    let api_response = response
        .json::<ApiResponse>()
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(api_response)
}

pub struct RgbModel {
    pub mapbox_token: String,
    pub api_rgb: String,
    pub data_elevation_covered: Vec<Vec<Vec<f64>>>,
}

impl RgbModel {

    //http://localhost:8080/api/terrain_rgb?lat=45.7716711&lon=4.8376036&radius=5&zoom=15&units_side=10000
    //point d'entrée du module
    pub async fn get_terrain(&mut self, params: &TerrainRequest) {

        //1 recuperer BBox
        let bbox: &Bbox = &params.bbox;//la reference est utiliser si on ne lutilise pas cela creer une copie
        //2 recuperer zoom position covered ==> les 81 tuiles .....
        let zoom_position_covered: Vec<Vec<u32>> = params.zoom_position_covered.clone();
        let zoom_position_elevation: Vec<Vec<u32>> = get_zoom_position_elevation(&zoom_position_covered);
        self.fetch(&zoom_position_covered, bbox).await;

        println!("grandparents uniques: {:?}", zoom_position_elevation);
    }

    pub async fn fetch( &mut self, zp_covered: &[Vec<u32>], bbox: &Bbox ) {
        let zoom_position_elevation = get_zoom_position_elevation(zp_covered);

        let fetch_futures = zoom_position_elevation.iter().map(|zoom_position| {
            let url =self.get_uri(&zoom_position).clone();
            println!("{:?}", url);
            async move {
                fetch_data(&url).await
            }
        });

        let tiles_results = join_all(fetch_futures).await;

        let new_data_segments = tiles_results
        .into_iter()
        .zip(zoom_position_elevation.iter())
        .map(|(tile_result, zoom_pos)| {
            let tile = tile_result.expect("Échec de la re cupération de la tuile");
            self.add_tile(&tile, zoom_pos, zp_covered, bbox)
        })
        .flatten()
        .collect::<Vec<_>>();
    }

    fn add_tile(&self, tile: &ApiResponse, zoom_pos: &Vec<u32>, zp_covered: &[Vec<u32>], bbox: &Bbox) -> Vec<i32> {
        let mut ret = Vec::new();

        ret.push(5);
        ret
    }

    fn get_uri(&self, zoom_pos: &[u32]) -> String {
        let [z, x, y] = zoom_pos else {
            panic!("zoom_pos doit contenir exactement trois éléments : [z, x, y]");
        };
    
        format!( "https://api.mapbox.com/v4/mapbox.terrain-rgb/{}/{}/{}@2x.pngraw?access_token={}", z, x, y, self.mapbox_token)
    }
}
