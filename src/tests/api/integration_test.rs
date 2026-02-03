use reqwest::Client;
use serde_json::Value;

use crate::tests::api::common::*;

#[tokio::test]
async fn test_signup() {
    let test_env = TestEnvironment::init("signup", 1).await;

    let client = &test_env.client;
    let base_url = &test_env.base_url;
    let token_id = &test_env.id_tokens[0];

    let res = signup(token_id, &client, &base_url).await;

    println!("Response: {:?}", res);
    assert!(res.status().is_success());
}

async fn signup(id_token: &str, client: &Client, base_url: &str) -> reqwest::Response {
    let res = client
        .post(base_url.to_owned() + "/auth/init")
        .bearer_auth(id_token)
        .send()
        .await
        .expect("Failed to send request");

    return res;
}

#[tokio::test]
async fn test_signup_then_me() {
    let test_env = TestEnvironment::init("signup_then_me", 2).await;

    let client = &test_env.client;
    let base_url = &test_env.base_url;

    let id_token_1 = &test_env.id_tokens[0];
    let id_token_2 = &test_env.id_tokens[1];

    let uid_1 = signup(id_token_1, client, base_url)
        .await
        .text()
        .await
        .unwrap();
    let uid_2 = signup(id_token_2, client, base_url)
        .await
        .text()
        .await
        .unwrap();

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

#[tokio::test]
async fn test_ping() {
    let test_env = TestEnvironment::init("ping", 0).await;

    let client = &test_env.client;
    let base_url = &test_env.base_url;

    let resp = client
        .get(base_url.to_owned() + "/ping")
        .send()
        .await
        .expect("Failed to send request");

    assert!(resp.status().is_success());
}

#[tokio::test]
async fn test_signup_then_get_account_settings() {
    let test_env = TestEnvironment::init("signup_then_get_account_settings", 1).await;

    let client = &test_env.client;
    let base_url = &test_env.base_url;

    let id_token_1 = &test_env.id_tokens[0];

    signup(id_token_1, client, base_url).await;

    let res1 = client
        .get(base_url.to_owned() + "/account/settings")
        .bearer_auth(id_token_1)
        .send()
        .await
        .expect("Failed to send request");

    assert!(res1.status().is_success());

    let text1 = res1.text().await.unwrap();
    let json_obj: Value = serde_json::from_str(&text1).unwrap();
    assert!(json_obj.get("preferred_command_style").is_some());
    assert!(json_obj.get("auto_commit_behaviour").is_some());
    assert!(json_obj.get("auto_push_behaviour").is_some());
    assert!(json_obj.get("auto_pull_behaviour").is_some());
}

#[tokio::test]
async fn test_signup_then_post_account_settings() {
    let test_env = TestEnvironment::init("signup_then_post_account_Settings", 1).await;

    let client = &test_env.client;
    let base_url = &test_env.base_url;

    let id_token_1 = &test_env.id_tokens[0];

    signup(id_token_1, client, base_url).await;

    let json_str = r#"
{
    "preferred_command_style": "unix",
    "auto_commit_behaviour": "timer",
    "auto_commit_timer_interval": 500,
    "auto_commit_count_interval": 0,
    "auto_pull_behaviour": "timer",
    "auto_push_behaviour": "count",
    "auto_push_timer_interval": 100,
    "auto_push_count_interval": 100,
    "auto_pull_timer_interval": 100
}"#;

    let json_body = serde_json::from_str::<Value>(&json_str).unwrap();

    let res1 = client
        .post(base_url.to_owned() + "/account/settings")
        .json(&json_body)
        .bearer_auth(id_token_1)
        .send()
        .await
        .expect("Failed to send request");

    assert!(res1.status().is_success());

    // TODO: don't require the entire settings object. Just specific fields you want to change.
    // This will be a pain to do because I either have to coalesce the values into the database, or
    // check the optional value for each field before making the DB call. I will also have to use
    // an UpdateSettingsParams struct instead of the domain model
    /*
    let json_str = r#"
{
    "preferred_command_style": "unix",
    "auto_commit_timer_interval": 500,
    "auto_commit_count_interval": 0,
    "auto_pull_behaviour": "timer",
    "auto_push_behaviour": null,
    "auto_push_timer_interval": 100,
    "auto_push_count_interval": 100,
    "auto_pull_timer_interval": null
}"#;

    let json_body = serde_json::from_str::<Value>(&json_str).unwrap();

    let res1 = client
        .post(base_url.to_owned() + "/account/settings")
        .json(&json_body)
        .bearer_auth(id_token_1)
        .send()
        .await
        .expect("Failed to send request");

    assert!(res1.status().is_success());
    */
}
