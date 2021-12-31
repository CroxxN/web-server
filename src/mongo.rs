use mongodb::{options::ClientOptions,Client};
pub use mongodb::bson::{doc, self};
use dotenv::dotenv;
use std::env;
use serde::{Serialize, Deserialize};
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct DB{
    pub client: Client,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct DataModel {
    title: String,
    content: String,
}

impl DB {
    pub async fn init()-> mongodb::error::Result<Self>{
        dotenv().ok();

        let db_user = env::var("MONGO_DB_USER").unwrap();
        let db_pass = env::var("MONGO_DB_PASS").unwrap();
        let mut client_options = ClientOptions::parse(format!("mongodb+srv://{}:{}@insults.l3jlv.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",db_user,db_pass))
        .await?;
        client_options.app_name = Some("insults".to_string());
        let client = Client::with_options(client_options)?;
        Ok(
            Self {
                client,
            }
        )
            
    }
    pub async fn get_insults(&self) -> mongodb::error::Result<Vec<DataModel>>{
        let insult_collection = self.client.database("insults").collection::<DataModel>("insults");
        
        let match_pipeline = doc!{
            "$match": {
                "title": "insult"
            }
        };
        let limit_pipeline = doc!{
            "$sample": {
                "size": 1
            }
        };

        let pipeline = vec![match_pipeline, limit_pipeline];
        let mut insult_response = insult_collection.aggregate(pipeline, None).await?;
        let mut response: Vec<DataModel> = Vec::new();
        while let Some(final_response) = insult_response.next().await {
            response.push(bson::from_document(final_response?)?);
        }
        Ok(response)
    }
    
}
