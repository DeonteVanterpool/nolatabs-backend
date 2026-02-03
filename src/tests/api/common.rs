use chrono::Utc;
use dotenvy::dotenv;
use std::env;

use firebase_auth::FirebaseAuth;

use fars::ApiKey;
use fars::Config;
use fars::Email;
use fars::Password;
use reqwest::Client;

pub struct TestEnvironment {
    pub firebase_auth: FirebaseAuth,
    pub id_tokens: Vec<String>,
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
    pub timestamp: i64,
}

impl TestEnvironment {
    pub async fn init(test_id: &str, count: u32) -> TestEnvironment {
        let firebase_auth = FirebaseAuth::new("nolatabs").await;
        let timestamp = Utc::now().timestamp_nanos_opt().unwrap();
        let mut id_tokens = Vec::new();
        for i in 0..count {
           id_tokens.push(signup_firebase(timestamp, &format!("{}_{}", test_id, i)).await);
        }
        let client = Client::new();
        let api_key = env::var("FIREBASE_API_KEY").expect(
            "Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env",
        );
        let base_url = load_base_url();
        return TestEnvironment {
            firebase_auth,
            client,
            id_tokens,
            base_url,
            timestamp,
            api_key,
        };
    }
}

async fn signup_firebase<'a>(ts: i64, ext: &'a str) -> String {
    // Load environment variables from .env file
    // Fails if no .env file found
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let api_key = env::var("FIREBASE_API_KEY").expect(
        "Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env",
    );
    let config = Config::new(ApiKey::new(api_key));

    let eml = ext.to_string() + "_" + &ts.to_string() + "@test.account";
    // 2. Sign up with email and password then get a session.
    let session = config
        .sign_up_with_email_password(Email::new(eml), Password::new("password"))
        .await
        .unwrap();

    // 3. Get user data through the session and get a new session.
    let (new_session, _user_data) = session.get_user_data().await.unwrap();
    return String::from(new_session.id_token.inner());
}

async fn signin_firebase(ts: i64) -> String {
    // Load environment variables from .env file
    // Fails if no .env file found
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let api_key = env::var("FIREBASE_API_KEY").expect(
        "Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env",
    );
    let config = Config::new(ApiKey::new(api_key));

    let eml = "deonte.vanterpool@test.account";
    // 2. Sign in with email and password then get a session.
    let session = config
        .sign_in_with_email_password(Email::new(eml), Password::new("password"))
        .await
        .unwrap();

    // 3. Get user data through the session and get a new session.
    let (new_session, _user_data) = session.get_user_data().await.unwrap();
    return String::from(new_session.id_token.inner());
}

fn load_base_url() -> String {
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let base_url = env::var("BASE_URL").expect(
        "Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env",
    );
    return base_url;
}
