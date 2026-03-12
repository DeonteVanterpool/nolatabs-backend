use stripe_checkout::CheckoutSessionMode;
use stripe_checkout::checkout_session::ProductData;
use stripe_checkout::checkout_session::{
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
};
use stripe_types::Currency;
use uuid::Uuid;

pub async fn start_checkout_session(
    client: &stripe::Client,
    user_id: Uuid,
    months: u32,
    subscription_type: SubscriptionType,
) -> Result<String, Box<dyn std::error::Error>> {
    let price = match subscription_type {
        SubscriptionType::CloudSync => 200,
        SubscriptionType::SyncCollaborate => 300,
    };
    let line_items = vec![CreateCheckoutSessionLineItems {
        quantity: Some(months.into()),
        price_data: Some(CreateCheckoutSessionLineItemsPriceData {
            currency: Currency::USD,
            product_data: Some(ProductData {
                name: product_name(&subscription_type),
                tax_code: None,
                description: None,
                images: None,
                metadata: None,
                unit_label: None,
            }),
            unit_amount: Some(price.into()),
            recurring: None,
            product: None,
            tax_behavior: None,
            unit_amount_decimal: None,
        }),
        ..Default::default()
    }];

    let checkout_session = CreateCheckoutSession::new()
        .mode(CheckoutSessionMode::Payment)
        .line_items(line_items)
        .metadata([
            (String::from("uuid"), user_id.to_string()),
            (
                "type".into(),
                match subscription_type {
                    SubscriptionType::CloudSync => "cloud",
                    SubscriptionType::SyncCollaborate => "collab",
                }
                .to_string(),
            ),
            (String::from("quantity"), months.to_string()),
        ])
        .success_url("https://deontevanterpool.com")
        .send(client)
        .await?;

    let url = checkout_session
        .url
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Missing checkout URL"))?;
    Ok(url)
}

fn product_name(subscription_type: &SubscriptionType) -> String {
    match subscription_type {
        SubscriptionType::CloudSync => "Cloud Sync -- Monthly".to_owned(),
        SubscriptionType::SyncCollaborate => "Sync Collaborate -- Monthly".to_owned(),
    }
}

use dotenvy::dotenv;
use stripe::Client;

use crate::models::account::SubscriptionType;
#[tokio::test]
pub async fn test_start_checkout_session() {
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let secret_key = std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);
    println!(
        "{}",
        start_checkout_session(&client, Uuid::new_v4(), 12, SubscriptionType::CloudSync)
            .await
            .unwrap()
    );
}

#[tokio::test]
pub async fn test_start_checkout_session_2() {
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let secret_key = std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);
    println!(
        "{}",
        start_checkout_session(
            &client,
            Uuid::new_v4(),
            6,
            SubscriptionType::SyncCollaborate
        )
        .await
        .unwrap()
    );
}
