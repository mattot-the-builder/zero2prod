use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // panic if can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // remove hardcoded port value and use value from the configuration
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
