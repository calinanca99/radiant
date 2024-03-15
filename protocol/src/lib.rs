pub use messages::*;

pub mod messages {
    use tonic::include_proto;

    include_proto!("messages");
}
