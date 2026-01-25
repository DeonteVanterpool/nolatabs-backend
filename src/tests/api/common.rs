use chrono::Utc;
use dotenvy::dotenv;
use std::env;

use firebase_auth::FirebaseAuth;

use fars::ApiKey;
use fars::Config;
use fars::Email;
use fars::Password;
use reqwest::Client;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref TEST_ENVIRONMENT: Mutex<TestEnvironment> = Mutex::new(TestEnvironment::new());
}

#[derive(Default)]
pub struct TestEnvironment {
    initialized: bool,
    pub firebase_auth: Option<FirebaseAuth>,
    pub id_token: Option<String>,
    pub client: Option<Client>,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub timestamp: Option<i64>,
}

impl TestEnvironment {
    fn new() -> Self {
        return TestEnvironment {
            initialized: false,
            ..Default::default()
        };
    }
    pub async fn init() -> std::sync::MutexGuard<'static, TestEnvironment> {
        let mut test_env = TEST_ENVIRONMENT.lock().unwrap();
        if test_env.initialized {
            return test_env;
        }
        let firebase_auth = FirebaseAuth::new("nolatabs").await;
        let ts = Utc::now().timestamp_millis();
        let id_token = signup_firebase(ts).await;
        let client = Client::new();
        let api_key = env::var("FIREBASE_API_KEY").expect(
            "Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env",
        );
        test_env.firebase_auth = Some(firebase_auth);
        test_env.id_token = Some(id_token);
        test_env.client = Some(client);
        test_env.api_key = Some(api_key);
        test_env.base_url = Some(load_base_url());
        test_env.initialized = true;
        test_env.timestamp = Some(ts);
        return test_env;
    }
}

async fn signup_firebase(ts: i64) -> String {
    // Load environment variables from .env file
    // Fails if no .env file found
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let api_key = env::var("FIREBASE_API_KEY").expect(
        "Could not find FIREBASE_API_KEY environment variable anywhere. Try putting it in .env",
    );
    let config = Config::new(ApiKey::new(api_key));

    let eml = ts.to_string() + "@test.account";
    // 2. Sign up with email and password then get a session.
    let session = config
        .sign_up_with_email_password(Email::new(eml), Password::new("password"))
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
