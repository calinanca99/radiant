use std::net::{TcpListener, TcpStream};

use radiant::connection_handler::ConnectionHandler;

fn handle_connection(stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Incoming connection: {}", peer_addr);

    let mut handler = ConnectionHandler::new(stream);

    loop {
        match handler.read_frame() {
            Ok(Some(frame)) => {
                println!("Client says: {frame}");
                if let Err(e) = handler.send_message("world") {
                    eprintln!("{e}");
                    continue;
                }
            }
            Ok(None) => {
                println!("Client {} disconnected", peer_addr);
                break;
            }
            Err(radiant::RadiantError::NetworkError(_)) => {
                eprintln!("Connection reset by peer");
                break;
            }
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("{e}"),
        };
    }
}
