use protocol::{radiant_client::RadiantClient, GetRequest, PingRequest, SetRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Customer {
    id: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RadiantClient::connect("http://127.0.0.1:3000").await?;

    // PING
    let request = tonic::Request::new(PingRequest {});
    let response = client.ping(request).await?.into_inner();
    println!("RESPONSE={:?}", response);

    // Set a value
    let customer = Customer {
        id: "customer-1".to_string(),
        age: 69,
    };
    let customer_bytes = bincode::serialize(&customer).unwrap();
    let request = tonic::Request::new(SetRequest {
        key: customer.id.clone(),
        data: customer_bytes,
    });
    let response = client.set(request).await?.into_inner();
    println!("RESPONSE={:?}", response);

    // Get a value
    let request = tonic::Request::new(GetRequest {
        key: customer.id.clone(),
    });
    let response = client.get(request).await?.into_inner();
    println!("RESPONSE={:?}", response);

    match response.result.unwrap() {
        protocol::get_response::Result::MaybeData(maybe_data) => {
            let bytes = maybe_data.data.unwrap().data;
            let fetched_customer: Customer = bincode::deserialize(&bytes).unwrap();

            assert_eq!(fetched_customer, customer);
        }
        protocol::get_response::Result::Error(e) => {
            eprintln!("{:?}", e)
        }
    }

    Ok(())
}
