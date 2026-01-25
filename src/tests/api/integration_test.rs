use crate::tests::api::common::*;

#[tokio::test]
async fn test_signup() {
    let test_env = TestEnvironment::init().await;

    let client = test_env.client.as_ref().unwrap();
    let base_url = test_env.base_url.as_ref().unwrap();
    let id_token = test_env.id_token.as_ref().unwrap();

    let res = client
        .post(base_url.to_owned() + "/auth/init")
        .bearer_auth(id_token)
        .send()
        .await
        .expect("Failed to send request");

    println!("Response: {:?}", res);

    assert!(res.status().is_success());
}
