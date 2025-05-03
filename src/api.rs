use std::sync::Mutex;
use uuid::Uuid;

use crate::blockchain::{self, Block, Blockchain};
use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self, block},
};
use serde::{Deserialize, Serialize};
struct AppState {
    blockchain: Mutex<Blockchain>,
    node_identifier: Mutex<Uuid>,
}

async fn test_api() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello World"))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        blockchain: Mutex::new(Blockchain::init()),
        node_identifier: Mutex::new(Uuid::new_v4()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/test_api", web::get().to(test_api))
            .route("/full_chain", web::get().to(full_chain))
            .route("/transactions/new", web::post().to(new_transaction))
            .route("/mine", web::get().to(mine))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Serialize)]
struct MineResponse {
    message: String,
    index: usize,
    transactions: Vec<blockchain::Transaction>,
    proof: u64,
    previous_hash: String,
}
pub async fn mine(data: web::Data<AppState>) -> impl Responder {
    // We run the proof of work algorithm to get the next proof...
    let mut blockchain = data.blockchain.lock().unwrap();
    let node_identifier = data.node_identifier.lock().unwrap();
    let last_block = blockchain.last_block();
    let last_block_proof = last_block.proof;
    let proof = blockchain.proof_of_work(last_block_proof);

    // We must receive a reward for finding the proof.
    // The sender is "0" to signify that this node has mined a new coin.
    blockchain.new_transaction(String::from("0"), node_identifier.to_string(), 1);

    // Forge the new Block by adding it to the chain
    let previous_hash = blockchain::block_hash(&last_block);
    let block = blockchain.new_block(proof, Some(previous_hash.clone()));

    let response = MineResponse {
        message: String::from("New Block Forged"),
        index: block.index,
        transactions: block.transactions.clone(),
        proof: proof,
        previous_hash: previous_hash,
    };

    HttpResponse::Ok().json(response)
}

pub async fn new_transaction(
    transaction: web::Json<blockchain::Transaction>,
    data: web::Data<AppState>,
) -> impl Responder {
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
