use std::sync::Mutex;

use crate::blockchain::Blockchain;
use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self, block},
};
use serde::{Deserialize, Serialize};
struct AppState {
    blockchain: Mutex<Blockchain>,
}

async fn test_api() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello World"))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        blockchain: Mutex::new(Blockchain::init()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/test_api", web::get().to(test_api))
            .route("/full_chain", web::get().to(full_chain))
            .route("/transactions/new", web::post().to(new_transaction))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub async fn mine() -> impl Responder {
    HttpResponse::Ok().body("We'll mine a new block")
}

#[derive(Debug, Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}
pub async fn new_transaction(
    transaction: web::Json<Transaction>,
    data: web::Data<AppState>,
) -> impl Responder {
    // println!("{:?}", transaction);
    let mut blockchain = data.blockchain.lock().unwrap();
    let index = blockchain.new_transaction(
        transaction.sender.clone(),
        transaction.recipient.clone(),
        transaction.amount,
    );
    HttpResponse::Ok().body(format!("Transaction will be added to Block {}", index))
}

pub async fn full_chain(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(&*blockchain)
}
