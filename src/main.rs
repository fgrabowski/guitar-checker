use actix_web::{
    error, get, post,
    web,
    App, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Wood {
    body: String,
    top: String,
    fretboard: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Guitar {
    id: String,
    model: String,
    construction: String,
    scale: f32,
    wood: Wood,
}

#[get("/")]
async fn get_all() -> Result<impl Responder> {
    let data = fs::read_to_string("src/data.json")?;
    let json: serde_json::Value =
        serde_json::from_str(&data)?;
    Ok(web::Json(json))
}

#[get("/{guitar_id}")]
async fn get_guitar(guitar_id: web::Path<String>) -> Result<impl Responder> {
    let data = fs::read_to_string("src/data.json")?;

    let deserialized: Vec<Guitar> = serde_json::from_str(&data)?;

    let matching_guitar = deserialized
        .into_iter()
        .find(|g| g.id == guitar_id.to_string());

    match matching_guitar {
        Some(matching_guitar) => Ok(web::Json(matching_guitar)),
        None => Err(error::ErrorNotFound(
            "Error: Guitar with specified ID doesn't exist.",
        )),
    }
}

#[post("/")]
async fn index(guitar: web::Json<Guitar>) -> Result<String> {
    Ok(format!("Added {}", guitar.id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all)
            .service(get_guitar)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
