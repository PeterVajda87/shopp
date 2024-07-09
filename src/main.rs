use openssl::ssl::{SslAcceptor, SslMethod};
use shopp::{
    settings::{RunMode, Settings},
    Run,
};
use std::net::TcpListener;
mod db;

pub mod settings;

use db::DB;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    let run_mode = RunMode::get();
    let settings = Settings::new(&run_mode).expect("Failed to parse settings."); 

    match run_mode {
        RunMode::Production => {
            let ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            ssl_builder.run(DB.clone(), settings)?.await
        }
        RunMode::Development => {
            let tcp_listener = TcpListener::bind(("0.0.0.0", settings.application_port)).unwrap();
            tcp_listener.run(DB.clone(), settings)?.await
        }
    }
}
