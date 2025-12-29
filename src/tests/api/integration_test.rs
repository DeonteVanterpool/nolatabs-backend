use std::collections::HashMap;
use dotenvy::dotenv;
use std::env;

use firebase_auth::FirebaseAuth;

use fars::Config;
use fars::ApiKey;
use reqwest::Client;
use fars::Email;
use fars::Password;

async fn signup_firebase() -> String {
    // Load environment variables from .env file
    // Fails if no .env file found
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let api_key = env::var("FIREBASE_API_KEY").expect("Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env");
    let config = Config::new(
        ApiKey::new(api_key),
    );
    
    // 2. Sign up with email and password then get a session.
    let session = config.sign_in_with_email_password(
        Email::new("deonte.vanterpool@outlook.com"),
        Password::new("password"),
    ).await.unwrap();

    // 3. Get user data through the session and get a new session.
    let (new_session, _user_data) = session.get_user_data().await.unwrap();
    return String::from(new_session.id_token.inner());
}

fn load_base_url() -> String {
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let base_url = env::var("BASE_URL").expect("Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env");
    return base_url;
}

#[tokio::test]
async fn test_signup() {
    let firebase_auth = FirebaseAuth::new("nolatabs").await;
    let id_token = signup_firebase().await;
    let client = Client::new();
    let base_url = load_base_url();
    let res = client.get(base_url + "/auth/init");
}
