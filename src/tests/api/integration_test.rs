use std::collections::HashMap;

use crate::FirebaseAuth;

use aws_config::imds::Client;
use axum::middleware::map_request_with_state;
use firebase_auth::FirebaseAuth;

use fars::Config;
use fars::ApiKey;
use fars::Email;
use fars::Password;

async fn signup_firebase() -> String {
    let config = Config::new(
        ApiKey::new("your-firebase-project-api-key"),
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

#[tokio::test]
async fn test_signup() {
    let firebase_auth = FirebaseAuth::new("nolatabs").await;
    let id_token = signup_firebase().await;
    let client = Client::builder().build();
    client.get("https://nolatabs.firebaseio.com/").await.unwrap();
}
