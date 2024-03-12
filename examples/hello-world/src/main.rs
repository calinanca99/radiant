use client::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Item {
    id: u64,
    data: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new("127.0.0.1:3000").await.unwrap();

    let item = Item {
        data: "Something".to_string(),
        id: 69420,
    };

    client.set(item.id, item.clone()).await.unwrap();

    let item: Item = client.get(item.id).await.unwrap();
    dbg!(item);

    Ok(())
}
