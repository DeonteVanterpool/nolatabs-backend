use reqwest::Client;

use crate::tests::api::common::*;

#[tokio::test]
async fn test_signup() {
    let test_env = TestEnvironment::init().await;

    let client = test_env.client.as_ref().unwrap();
    let base_url = test_env.base_url.as_ref().unwrap();
    let token_id = test_env.id_token_signup.as_ref().unwrap();

    let res = signup(token_id, &client, &base_url).await;

    println!("Response: {:?}", res);
    assert!(res.status().is_success());
}

async fn signup<'a>(id_token: &'a str, client: &Client, base_url: &str) -> reqwest::Response {
    let res = client
        .post(base_url.to_owned() + "/auth/init")
        .bearer_auth(id_token)
        .send()
        .await
        .expect("Failed to send request");

    return res;
}

#[tokio::test]
async fn test_me() {
    let test_env = TestEnvironment::init().await;

    let client = test_env.client.as_ref().unwrap();
    let base_url = test_env.base_url.as_ref().unwrap();

    let id_token_1 = test_env.id_token_me_1.as_ref().unwrap();
    let id_token_2 = test_env.id_token_me_2.as_ref().unwrap();

    let uid_1 = signup(id_token_1, client, base_url).await.text().await.unwrap();
    let uid_2 = signup(id_token_2, client, base_url).await.text().await.unwrap();

    let res1 = client
        .get(base_url.to_owned() + "/auth/me")
        .bearer_auth(id_token_1)
        .send()
        .await
        .expect("Failed to send request");

    let res2 = client
        .get(base_url.to_owned() + "/auth/me")
        .bearer_auth(id_token_2)
        .send()
        .await
        .expect("Failed to send request");

    println!("Response 1: {:?}", res1);
    println!("Response 2: {:?}", res2);

    assert!(res1.status().is_success());
    assert!(res2.status().is_success());

    let text1 = res1.text().await.unwrap();
    let text2 = res2.text().await.unwrap();

    // Assert valid UUIDs
    assert!(uuid::Uuid::parse_str(&text1).is_ok());
    assert!(uuid::Uuid::parse_str(&text2).is_ok());

    // Assert they are different
    assert_ne!(text1, text2);

    assert_eq!(text1, uid_1);
    assert_eq!(text2, uid_2);
}

