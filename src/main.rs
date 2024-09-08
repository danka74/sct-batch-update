#![allow(unused)]

use axum::{
    routing::{get, post}, Json, Router
};

mod error;
mod model;
mod controller;

use controller::process_batch;
pub use error::{Error, Result};
use reqwest::Client;

#[tokio::main]
async fn main() {

    let client = Client::new();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api", get(||  async { Json(model::BatchChangeRequest::default())}))
        .route("/process", post(process_batch))
        .with_state(client);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Server started!");
}
