use migration::MigratorTrait;
use openssl::ssl::{SslAcceptor, SslMethod};
use shopp::db::DB;
use shopp::{settings::RunMode, Run, RUN_MODE, SETTINGS};
use std::net::TcpListener;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    migration::Migrator::fresh(&*DB).await.unwrap();

    match *RUN_MODE {
        RunMode::Production => {
            let ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            ssl_builder.run(&*SETTINGS)?.await
        }
        RunMode::Development => {
            let tcp_listener =
                TcpListener::bind(("0.0.0.0", (*SETTINGS).application_port)).unwrap();
            tcp_listener.run(&*SETTINGS)?.await
        }
    }
}
