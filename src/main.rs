use tide::prelude::*;

use poke_spearify::server;
use poke_spearify::wrappers::pokemon;
use poke_spearify::wrappers::shakespeare;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    tide::log::start();

    let shakespeare_wrapper = shakespeare::ShakespeareWrapper::new();
    let pokemon_wrapper = pokemon::PokemonWrapper::new();

    let app = server::build(shakespeare_wrapper, pokemon_wrapper).await;

    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("5000"));
    let mut listener = app
        .bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Error: Can't bind the port");

    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }
    listener.accept().await.unwrap();
}
