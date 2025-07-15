use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    pub properties: Properties,
    #[serde(rename = "type")]
    pub _type: String,
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonFeature {
    #[serde(rename = "type")]
    pub _type: String,
    geometry : Geometry,
}

#[derive(Debug)]
pub struct BoundingBox {
    pub north_west: [f64; 2],
    pub south_east: [f64; 2],
}
