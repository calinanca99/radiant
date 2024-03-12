use server::{Connection, Db};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(stream: TcpStream, db: Db) -> server::Result<()> {
    let mut handler = Connection::new(stream, db);
    handler.process().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let db = Db::new();

    loop {
        let db = db.clone();
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, db).await {
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
