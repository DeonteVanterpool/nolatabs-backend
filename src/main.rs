use crate::state::AppState;
use aws_config::BehaviorVersion;
use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use std::future::IntoFuture;

pub mod handlers;
pub mod models;
pub mod services;
pub mod state;
pub mod tests;

struct Environment {
}

impl Environment {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file
    // Fails if no .env file found
    if dotenv().is_err() {
        println!("no .env file found...")
    }

    let config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
    let client = aws_sdk_s3::Client::new(&config);

    let env = Environment::new();

    // build our application with a route
    let app = Router::new().route("/ping", get(handlers::status::ping));
        // .route("/subcription", post(handlers::post_subcription))

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3892")
        .await
        .unwrap();

    println!(
        "Serving public API: http://{}",
        listener.local_addr().unwrap()
    );

    let public_api = axum::serve(listener, app).into_future();

    // run both APIs concurrently
    let public_handle = tokio::spawn(public_api);

    // Wait for either API to finish
    tokio::select! {
        _ = public_handle => {},
    }

    println!("Shutting down the server...");
}

