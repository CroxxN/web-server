mod mongo;
use mongo::{DB, DataModel};
use std::error::Error;

use actix_web::{get, Error as actixError , App, HttpServer, HttpResponse, Result};

use serde::Serialize;


#[derive(Serialize)]
struct Insults{
    insult: String,
}

#[get("/api/insults")]
async fn insult()-> Result<HttpResponse, actixError>{
    let response_insult:Vec<DataModel> = get_insult().await.unwrap();
    Ok(HttpResponse::Ok().json(response_insult))
    
}

async fn get_insult() -> Result<Vec<mongo::DataModel>, Box<dyn Error>> {
    let mongo_client = DB::init().await?;
    let insult_response: Vec<DataModel> = mongo_client.get_insults().await?;
    Ok(insult_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Running");
    HttpServer::new(|| {
        App::new().service(insult)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}

