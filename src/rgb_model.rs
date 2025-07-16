use actix_web::{error::ErrorInternalServerError, Error};
use geo::{Destination, Haversine};
use  geo_types::{Point};
use crate::models::{ApiResponse, Bbox, TerrainRequest};
use std::collections::HashMap;

const CONST_VERTICES: usize = 128;

//http://localhost:8080/api/terrain_rgb?lat=45.7716711&lon=4.8376036&radius=5&zoom=15&units_side=10000
//point d'entrée du module
pub fn get_terrain( params: &TerrainRequest ) {

    //1 recuperer BBox
    let bbox: &Bbox = &params.bbox;//la reference est utiliser si on ne lutilise pas cela creer une copie
    //2 recuperer zoom position covered ==> les 81 tuiles .....
    let zoom_position_covered: Vec<Vec<u32>> = params.zoom_position_covered.clone();
    let zoom_position_elevation: Vec<Vec<u32>> = get_zoom_position_elevation(&zoom_position_covered);

    println!("grandparents uniques: {:?}", zoom_position_elevation);
}

fn get_unit_per_meter( units_side: u32, radius: u32 ) -> f64 {
    let units_side_f: f64 = units_side as f64;
    let radius_f: f64 = radius as f64;

    units_side_f / ( radius_f * 2.0_f64.sqrt()  * 1000.0 )
}

pub fn add_tile() {

}

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


async fn fetch_data( url: &str ) -> Result<ApiResponse, Error> {
    let response = reqwest::get(url)
        .await
        .map_err(ErrorInternalServerError)?;
    let api_response = response
        .json::<ApiResponse>()
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(api_response)
}
