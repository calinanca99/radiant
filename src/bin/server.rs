use radiant::Connection;
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(stream: TcpStream) -> radiant::Result<()> {
    let mut handler = Connection::new(stream);
    handler.process().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream).await {
                        eprintln!("{e}");
                    }
                });
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}
