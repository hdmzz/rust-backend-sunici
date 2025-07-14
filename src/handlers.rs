use actix_web::web::Query;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder, Result};
use crate::models::{User, CreateUserRequest, UserResponse};
use crate::rgb_model::{self, get_terrain, TerrainRgbParams};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/users")]
pub async fn create_user(user_data: web::Json<CreateUserRequest>) -> Result<impl Responder> {
    let generated_id = 42;
    
    let response = UserResponse {
        id: generated_id,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
        message: Some("Utilisateur créé avec succès".to_string()),
    };
    
    Ok(HttpResponse::Created().json(response))
}

//#[put("/users/{id}")]
//pub async fn update_user(
//    path: web::Path<u32>,
//    user_data: web::Json<CreateUserRequest>
//) -> Result<impl Responder> {
//    let user_id = path.into_inner();
//    let updated_user = User {
//        id: user_id,
//        name: user_data.name.clone(),
//        email: user_data.email.clone(),
//    };
//    Ok(HttpResponse::Ok().json(updated_user))
//}

//#[delete("/users/{id}")]
//pub async fn delete_user(path: web::Path<u32>) -> Result<impl Responder> {
//    let user_id = path.into_inner();
//    Ok(HttpResponse::Ok().json(format!("User {} deleted", user_id)))
//}

//#[get("/users/{user_id}/posts/{post_id}")]
//pub async fn get_user_post(path: web::Path<(u32, u32)>) -> Result<impl Responder> {
//    let (user_id, post_id) = path.into_inner();
//    Ok(HttpResponse::Ok().json(format!("User {} Post {}", user_id, post_id)))
//}

#[get("/api/terrain_rgb")]
pub async fn get_terrain_rgb(params: web::Query<TerrainRgbParams>) -> String {
    let params_terrain: TerrainRgbParams = params.into_inner();

    let _result_data = rgb_model::get_terrain(&params_terrain);

    let ret: String = String::new();
    ret
}
