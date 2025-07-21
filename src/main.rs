use actix_cors::Cors;
use actix_web::{middleware::Compress, App, HttpServer};
mod routes;
mod handlers;
mod models;
mod rgb_model;
mod projection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Démarrage du serveur sur http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        let cors: Cors = Cors::default()
            .allow_any_origin()    // Autorise les requêtes de n'importe quelle origine
            .allow_any_method()    // Autorise toutes les méthodes HTTP (GET, POST, etc.)
            .allow_any_header()    // Autorise tous les en-têtes HTTP
            .expose_headers(vec!["X-Tile-Metadata"])
            .max_age(3600);  

        App::new()
        .wrap(cors)
        .wrap(Compress::default())
        .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .workers(1) 
    .run()
    .await
}


//fn main() {
//    let lat = 45.7716768;
//    let lon = 4.8376049;
//    let radius_km = 5.0;
//    let radius_meters = radius_km * 1000.0;

//    let bbox = calculate_bbox(lat, lon, radius_meters);

//    println!("Bounding Box calculée :");
//    println!("North-West: lon={}, lat={}", bbox.north_west[0], bbox.north_west[1]);
//    println!("South-East: lon={}, lat={}", bbox.south_east[0], bbox.south_east[1]);

//    // Pour un affichage similaire à votre JSON
//    println!("\n--- Exemple de sortie JSON ---");
//    println!("{{");
//    println!("    \"northWest\": [");
//    println!("        {},", bbox.north_west[0]);
//    println!("        {}", bbox.north_west[1]);
//    println!("    ],");
//    println!("    \"southEast\": [");
//    println!("        {},", bbox.south_east[0]);
//    println!("        {}", bbox.south_east[1]);
//    println!("    ]");
//    println!("}}");
//}
