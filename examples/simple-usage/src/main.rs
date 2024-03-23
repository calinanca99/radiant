use client::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Customer {
    id: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("http://127.0.0.1:3000").await?;

    // PING
    let response = client.ping().await?;
    println!("RESPONSE={:?}", response);

    // Set a value
    let customer = Customer {
        id: "customer-1".to_string(),
        age: 69,
    };
    client.set(customer.id.clone(), &customer).await?;

    // Get a value
    let fetched_customer = client.get::<Customer>(customer.id.clone()).await?.unwrap();
    assert_eq!(fetched_customer, customer);

    Ok(())
}
