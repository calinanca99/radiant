use radiant_client::Client;
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
    client.set(&customer.id, &customer).await?;

    // Get a value
    let fetched_customer = client.get::<Customer>(&customer.id).await?.unwrap();
    dbg!(&fetched_customer);
    assert_eq!(fetched_customer, customer);

    // Delete a value
    client.del(&customer.id).await?;
    let fetched_customer = client.get::<Customer>(&customer.id).await?;
    dbg!(&fetched_customer);

    let res = client.del(&customer.id).await;
    dbg!(&res);

    Ok(())
}
