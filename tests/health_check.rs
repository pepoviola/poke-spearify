use poke_spearify::server;
use poke_spearify::wrappers::pokemon;
use poke_spearify::wrappers::shakespeare;

#[async_std::test]
async fn health_check() -> tide::Result<()> {
    dotenv::dotenv().ok();
    let shakespeare_wrapper = shakespeare::ShakespeareWrapper::new();
    let pokemon_wrapper = pokemon::PokemonWrapper::new();
    let app = server::build(shakespeare_wrapper, pokemon_wrapper).await;

    let res = surf::Client::with_http_client(app)
        .get("https://example.com/health_check")
        .await?;

    assert_eq!(200, res.status());
    Ok(())
}
