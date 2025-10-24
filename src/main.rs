use crate::state::AppState;
use aws_config::BehaviorVersion;
use axum::{
    Router,
    response::Redirect,
    routing::{get, post},
};
use dotenvy::dotenv;
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use sqlx::pool::PoolOptions;
use std::future::IntoFuture;
use std::{env, sync::Arc};

pub mod handlers;
pub mod logic;
pub mod models;
pub mod repository;
pub mod state;
pub mod tests;

struct Environment {
    database_url: String,
    firebase_project_id: String,
}

impl Environment {
    pub fn initialize() -> Self {
        // Load environment variables from .env file
        // Fails if no .env file found
        if dotenv().is_err() {
            println!("no .env file found...")
        }

        Self {
            database_url: env::var("DATABASE_URL").expect(
                "Could not find DATABASE_URL environment variable anywhere. Try putting it in .env",
            ),
            firebase_project_id: env::var("FIREBASE_PROJECT_ID").expect(
                "Could not find FIREBASE_PROJECT_ID environment variable anywhere. Try putting it in .env",
            ),
        }
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
    let client = aws_sdk_s3::Client::new(&config);
    let env = Environment::initialize();

    println!("db_url: {}...", &env.database_url[..13]);
    println!("fb_id: {}...", &env.firebase_project_id[..5]);
    let pool = PoolOptions::new().connect(&env.database_url).await.expect(
        &format!("Could not connect to the database. Please check your DATABASE_URL environment variable: {}",
        &env.database_url
    ));
    let firebase_auth = Arc::new(FirebaseAuth::new(&env.firebase_project_id).await);

    // build our application with a route
    let app = Router::new()
        .route("/ping", get(handlers::status::ping))
        .with_state(AppState::new(pool))
        .with_state(FirebaseAuthState { firebase_auth });
    // .route("/subcription", post(handlers::post_subcription))

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3892").await.unwrap();

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
