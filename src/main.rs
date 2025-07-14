use actix_web::{App, HttpServer};
mod routes;
mod handlers;
mod models;
mod rgb_model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Démarrage du serveur sur http://127.0.0.1:8080");
    println!("Configuration pour réutiliser le port (équivalent SO_REUSEADDR)");
    
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .workers(1) 
    .run()
    .await
}
