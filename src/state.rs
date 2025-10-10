use futures::future;
use std::error::Error;

use crate::models;

#[derive(Debug, Clone)]
pub struct AppState {
    auth: firebase_auth_sdk::Auth,
}

impl AppState {}
