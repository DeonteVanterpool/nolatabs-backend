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
    product_name: &str,
    amount_cents: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let line_items = vec![CreateCheckoutSessionLineItems {
        quantity: Some(1),
        price_data: Some(CreateCheckoutSessionLineItemsPriceData {
            currency: Currency::USD,
            product_data: Some(ProductData {
                name: product_name.into(),
                tax_code: None,
                description: None,
                images: None,
                metadata: None,
                unit_label: None,
            }),
            unit_amount: Some(amount_cents.into()), // total = monthly_price * months
            recurring: None,
            product: None,
            tax_behavior: None,
            unit_amount_decimal: None,
        }),
        ..Default::default()
    }];

    let checkout_session = CreateCheckoutSession::new()
        .mode(CheckoutSessionMode::Payment)
        .payment_intent_data(CreateCheckoutSessionPaymentIntentData)
        .line_items(line_items)
        .metadata([(String::from("uuid"), user_id.to_string())])
        .success_url("https://deontevanterpool.com")
        .send(client)
        .await?;

    let url = checkout_session
        .url
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Missing checkout URL"))?;
    Ok(url)
}

use stripe::Client;
use dotenvy::dotenv;
#[tokio::test]
pub async fn test_start_checkout_session() {
    if dotenv().is_err() {
        println!("no .env file found...")
    }
    let secret_key = std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);
    println!("{}", start_checkout_session(&client, Uuid::new_v4(), "6 Months Collab + Sync", 200 * 6).await.unwrap());
}
