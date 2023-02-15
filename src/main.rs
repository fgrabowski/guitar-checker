use std::fs;
use actix_web::{get, post, web::{self}, HttpResponse, HttpServer, Responder, App, Result, guard::Get};
use serde::{Serialize, Deserialize};


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
    let data = fs::read_to_string("src/data.json")
        .expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");
    let deserialized: Vec<Guitar> = serde_json::from_str(&json.to_string()).unwrap();
    Ok(web::Json(deserialized))
}

// #[get("/{guitar_id}")]
// async fn get_guitar(guitar_id: web::Path<u32>) -> Result<impl Responder> {
//     let data = fs::read_to_string("src/data.json")
//         .expect("Unable to read file");
//     let json: serde_json::Value = serde_json::from_str(&data)
//         .expect("JSON does not have correct format.");

//     let v = json!()
//     Ok(web::Json(json))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}