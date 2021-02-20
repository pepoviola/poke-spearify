use tide::prelude::*;

use poke_spearify::server;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    tide::log::start();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let app = server::build().await;
    let mut listener = app
        .bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Error: Can't bind the port");

    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }
    listener.accept().await.unwrap();
}

