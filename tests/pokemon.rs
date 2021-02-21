use tide::prelude::json;

use poke_spearify::server;
use poke_spearify::wrappers::pokemon;
use poke_spearify::wrappers::shakespeare;

use poke_spearify::controllers::pokemon::PokemonResponse;

use assert_json_diff::assert_json_eq;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[async_std::test]
async fn get_pokemon() -> tide::Result<()> {
    dotenv::dotenv().ok();

    // arrenge wrappers mocks
    let mock_pokemon_server = MockServer::start().await;
    let existing_pokemon = "charizard";
    let mock_path = format!(
        "{}{}",
        pokemon::POKEMON_SERVICE_PATH,
        existing_pokemon.to_string()
    );

    const CHARIZARD_CONTENT: &str = include_str!("../samples/charizard.json");

    let charizard_as_json: serde_json::Value = serde_json::from_str(&CHARIZARD_CONTENT).unwrap();
    let response = ResponseTemplate::new(200).set_body_json(charizard_as_json);

    Mock::given(method("GET"))
        .and(path(&mock_path))
        .respond_with(response)
        .mount(&mock_pokemon_server)
        .await;

    let mock_translation_server = MockServer::start().await;

    const TRASLATION_CONTENT: &str = include_str!("../samples/shakespeare_translation.json");
    let translation_as_json: serde_json::Value = serde_json::from_str(&TRASLATION_CONTENT).unwrap();
    let response = ResponseTemplate::new(200).set_body_json(&translation_as_json);

    Mock::given(method("POST"))
        .and(path(shakespeare::TRANSLATION_SHAKESPEARE_PATH))
        .respond_with(response)
        .mount(&mock_translation_server)
        .await;

    // act
    let shakespeare_wrapper =
        shakespeare::ShakespeareWrapper::with_base_url(&mock_translation_server.uri());
    let pokemon_wrapper = pokemon::PokemonWrapper::with_base_url(&mock_pokemon_server.uri());
    let app = server::build(shakespeare_wrapper, pokemon_wrapper).await;

    let pokemon_url = format!("https://example.com/pokemon/{}", existing_pokemon);
    let mut res = surf::Client::with_http_client(app).get(pokemon_url).await?;

    // assert
    assert_eq!(200, res.status());

    let pokemon_response: PokemonResponse = res.body_json().await?;
    assert_json_eq!(
        pokemon_response,
        json!({
            "name": "charizard",
            "description": "Rust, a language empowering everyone to buildeth reliable and efficient software."
        })
    );

    Ok(())
}

#[async_std::test]
async fn get_pokemon_with_api_key() -> tide::Result<()> {
    dotenv::dotenv().ok();

    // arrenge wrappers mocks
    let mock_pokemon_server = MockServer::start().await;
    let existing_pokemon = "charizard";
    let mock_path = format!(
        "{}{}",
        pokemon::POKEMON_SERVICE_PATH,
        existing_pokemon.to_string()
    );

    const CHARIZARD_CONTENT: &str = include_str!("../samples/charizard.json");

    let charizard_as_json: serde_json::Value = serde_json::from_str(&CHARIZARD_CONTENT).unwrap();
    let response = ResponseTemplate::new(200).set_body_json(charizard_as_json);

    Mock::given(method("GET"))
        .and(path(&mock_path))
        .respond_with(response)
        .mount(&mock_pokemon_server)
        .await;

    let mock_translation_server = MockServer::start().await;

    const TRASLATION_CONTENT: &str = include_str!("../samples/shakespeare_translation.json");
    let translation_as_json: serde_json::Value = serde_json::from_str(&TRASLATION_CONTENT).unwrap();
    let response = ResponseTemplate::new(200).set_body_json(&translation_as_json);

    Mock::given(method("POST"))
        .and(path(shakespeare::TRANSLATION_SHAKESPEARE_PATH))
        .respond_with(response)
        .mount(&mock_translation_server)
        .await;

    // act
    let mut shakespeare_wrapper =
        shakespeare::ShakespeareWrapper::with_base_url(&mock_translation_server.uri());
    shakespeare_wrapper.set_api_key(Some("secret".to_string()));

    let pokemon_wrapper = pokemon::PokemonWrapper::with_base_url(&mock_pokemon_server.uri());
    let app = server::build(shakespeare_wrapper, pokemon_wrapper).await;

    let pokemon_url = format!("https://example.com/pokemon/{}", existing_pokemon);
    let mut res = surf::Client::with_http_client(app).get(pokemon_url).await?;

    // assert
    assert_eq!(200, res.status());

    let pokemon_response: PokemonResponse = res.body_json().await?;
    assert_json_eq!(
        pokemon_response,
        json!({
            "name": "charizard",
            "description": "Rust, a language empowering everyone to buildeth reliable and efficient software."
        })
    );

    Ok(())
}

#[async_std::test]
async fn get_non_existing_pokemon() -> tide::Result<()> {
    dotenv::dotenv().ok();

    // arrenge wrappers mocks
    let mock_pokemon_server = MockServer::start().await;
    let existing_pokemon = "nocharizard";
    let mock_path = format!(
        "{}{}",
        pokemon::POKEMON_SERVICE_PATH,
        existing_pokemon.to_string()
    );

    Mock::given(method("GET"))
        .and(path(&mock_path))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_pokemon_server)
        .await;

    let mock_translation_server = MockServer::start().await;

    const TRASLATION_CONTENT: &str = include_str!("../samples/shakespeare_translation.json");
    let translation_as_json: serde_json::Value = serde_json::from_str(&TRASLATION_CONTENT).unwrap();
    let response = ResponseTemplate::new(200).set_body_json(&translation_as_json);

    Mock::given(method("POST"))
        .and(path(shakespeare::TRANSLATION_SHAKESPEARE_PATH))
        .respond_with(response)
        .mount(&mock_translation_server)
        .await;

    // act
    let shakespeare_wrapper =
        shakespeare::ShakespeareWrapper::with_base_url(&mock_translation_server.uri());
    let pokemon_wrapper = pokemon::PokemonWrapper::with_base_url(&mock_pokemon_server.uri());
    let app = server::build(shakespeare_wrapper, pokemon_wrapper).await;

    let pokemon_url = format!("https://example.com/pokemon/{}", existing_pokemon);
    let res = surf::Client::with_http_client(app).get(pokemon_url).await?;

    // assert
    assert_eq!(404, res.status());

    Ok(())
}
