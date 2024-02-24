use shopp::run;
use std::net::TcpListener;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a port");
    run(listener)?.await
}
