use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;


#[derive(Serialize)]
struct Insults{
    insult: String,
}

#[get("/insults")]
async fn insult()-> Result<impl Responder>{
    let insult = Insults {
        insult: String::from("hello"),
    };
    Ok(web::Json(insult))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(insult)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

