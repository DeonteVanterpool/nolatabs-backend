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
use aws_sdk_secretsmanager as secretsmanager;

pub mod handlers;
pub mod logic;
pub mod models;
pub mod repository;
pub mod state;
pub mod tests;

#[derive(Clone)]
struct Environment {
    database_user: String,
    database_secret_arn: String,
    database_host: String,
    database_port: String,
    database_name: String,
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
            database_user: env::var("DB_USER").expect(
                "Could not find DB_USER environment variable anywhere. Try putting it in .env",
            ),
            database_secret_arn: env::var("DB_SECRET_ARN").expect(
                "Could not find DB_SECRET_ARN environment variable anywhere. Try putting it in .env",
            ),
            database_host: env::var("DB_HOST").expect(
                "Could not find DB_HOST environment variable anywhere. Try putting it in .env",
            ),
            database_port: env::var("DB_PORT").expect(
                "Could not find DB_PORT environment variable anywhere. Try putting it in .env",
            ),
            database_name: env::var("DB_NAME").expect(
                "Could not find DB_NAME environment variable anywhere. Try putting it in .env",
            ),
            firebase_project_id: env::var("FIREBASE_PROJECT_ID").expect(
                "Could not find FIREBASE_PROJECT_ID environment variable anywhere. Try putting it in .env",
            ),
        }
    }
}

fn db_connection_string(env: &Environment, secret: String) -> String {
    format!(
        "postgresql://{}:{}@{}:{}/{}",
        env.database_user,
        secret,
        env.database_host,
        env.database_port,
        env.database_name
    )
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config = aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await;
    let client = aws_sdk_s3::Client::new(&config);
    let env = Environment::initialize();
    let secrets_manager_client = aws_sdk_secretsmanager::Client::new(&config);
    let builder = secrets_manager_client.get_secret_value();
    let db_password = String::from(builder.set_secret_id(Some(env.database_secret_arn.clone())).send().await.expect("Error getting secret").secret_string().expect("Error getting secret string"));

    let pool = PoolOptions::new().connect(&db_connection_string(&env, db_password)).await.expect(
        &format!("Could not connect to the database. Please check your DATABASE_URL environment variable.",
    ));

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Could not run database migrations");
    let firebase_auth = Arc::new(FirebaseAuth::new(&env.firebase_project_id).await);

    // build our application with a route
    let app = Router::new()
        .route("/ping", get(handlers::status::ping))
        .route("/auth/init", post(handlers::auth::init))
        .route("/auth/me", get(handlers::auth::me))
        .route("/account/settings", get(handlers::account::get_settings))
        .route("/account/settings", post(handlers::account::post_settings))
        .with_state(AppState::new(pool, firebase_auth));
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
