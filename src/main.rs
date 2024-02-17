use shopp::run;

#[ntex::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
