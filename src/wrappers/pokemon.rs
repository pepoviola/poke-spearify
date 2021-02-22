use crate::wrappers::errors::WrapperError;
use serde::Deserialize;

const POKEMON_SERVICE_URI: &str = "https://pokeapi.co";
pub const POKEMON_SERVICE_PATH: &str = "/api/v2/pokemon-species/";
#[derive(Deserialize, Debug)]
struct Pokemon {
    flavor_text_entries: Vec<FlavorText>,
}

#[derive(Deserialize, Debug)]
struct FlavorText {
    flavor_text: String,
    language: Language,
}

#[derive(Deserialize, Debug)]
struct Language {
    name: String,
}

#[derive(Clone, Debug)]
pub struct PokemonWrapper {
    base_url: String,
}

impl PokemonWrapper {
    pub fn new() -> Self {
        Self {
            base_url: POKEMON_SERVICE_URI.to_string(),
        }
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_description(&self, pokemon_name: &str) -> Result<String, WrapperError> {
        let pokemon_url = format!("{}{}{}", self.base_url, POKEMON_SERVICE_PATH, pokemon_name);
        let pokemon = fetch_pokemon(&pokemon_url).await?;
        let description = pokemon.get_description()?;
        Ok(description)
    }
}

impl Default for PokemonWrapper {
    fn default() -> Self {
        PokemonWrapper::new()
    }
}

impl Pokemon {
    fn get_description(&self) -> Result<String, WrapperError> {
        let description = self
            .flavor_text_entries
            .iter()
            .find(|desc| desc.language.name == "en")
            .ok_or(WrapperError::NoDescription)?;

        // The api description is multiline with `\n` and also contains `\u{c}` sequence
        // here we parse the description to be one line without the `\u{c}` sequence.
        let parsed_description = description
            .flavor_text
            .lines()
            .map(|line| line.trim().replace("\u{c}", " "))
            .collect::<Vec<String>>()
            .join(" ");
        Ok(parsed_description)
    }
}

async fn fetch_pokemon(pokemon_url: &str) -> Result<Pokemon, WrapperError> {
    let mut res = surf::get(pokemon_url).await.map_err(|e| {
        tide::log::error!("Error: {}, getting response from Pokemon API", e);
        WrapperError::UnexpectedError
    })?;

    let status: u16 = res.status().into();
    match status {
        200 => {
            let pokemon: Pokemon = res.body_json().await.map_err(|e| {
                tide::log::error!("Error: {}, deserializing response to Pokemon", e);
                WrapperError::ParsingError
            })?;
            Ok(pokemon)
        }
        404 => Err(WrapperError::NotFound),
        _ => Err(WrapperError::UnexpectedError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[async_std::test]
    async fn fetch_pokemon_description() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;
        let existing_pokemon = "charizard";
        let mock_path = format!("{}{}", POKEMON_SERVICE_PATH, existing_pokemon.to_string());

        const CHARIZARD_CONTENT: &str = include_str!("../../samples/charizard.json");

        let charizard_as_json: serde_json::Value =
            serde_json::from_str(&CHARIZARD_CONTENT).unwrap();
        let response = ResponseTemplate::new(200).set_body_json(charizard_as_json);

        Mock::given(method("GET"))
            .and(path(&mock_path))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let pokemon_url = format!("{}{}", &mock_server.uri(), &mock_path);
        let pokemon = fetch_pokemon(&pokemon_url).await?;

        let description = pokemon.get_description()?;
        assert_eq!(description, "Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.");

        Ok(())
    }

    #[async_std::test]
    async fn fetch_non_existing_pokemon_description() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;
        let non_existing_pokemon = "nocharizard";
        let mock_path = format!(
            "{}{}",
            POKEMON_SERVICE_PATH,
            non_existing_pokemon.to_string()
        );

        Mock::given(method("GET"))
            .and(path(&mock_path))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let pokemon_url = format!("{}{}", &mock_server.uri(), &mock_path);
        let pokemon = fetch_pokemon(&pokemon_url).await;

        assert_eq!(true, pokemon.is_err());

        assert_eq!(WrapperError::NotFound, pokemon.err().unwrap());

        Ok(())
    }

    #[async_std::test]
    async fn fetch_pokemon_without_description() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;
        let existing_pokemon = "charizard";
        let mock_path = format!("{}{}", POKEMON_SERVICE_PATH, existing_pokemon.to_string());

        const CHARIZARD_CONTENT: &str = include_str!("../../samples/charizard_without_desc.json");

        let charizard_as_json: serde_json::Value =
            serde_json::from_str(&CHARIZARD_CONTENT).unwrap();
        let response = ResponseTemplate::new(200).set_body_json(charizard_as_json);

        Mock::given(method("GET"))
            .and(path(&mock_path))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let pokemon_url = format!("{}{}", &mock_server.uri(), &mock_path);
        let pokemon = fetch_pokemon(&pokemon_url).await?;

        let description = pokemon.get_description();

        assert_eq!(true, description.is_err());

        assert_eq!(WrapperError::NoDescription, description.err().unwrap());

        Ok(())
    }

    #[async_std::test]
    async fn fetch_pokemon_parse_error() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;
        let existing_pokemon = "charizard";
        let mock_path = format!("{}{}", POKEMON_SERVICE_PATH, existing_pokemon.to_string());

        const CHARIZARD_CONTENT: &str = include_str!("../../samples/charizard_bad.json");

        let response = ResponseTemplate::new(200).set_body_json(CHARIZARD_CONTENT);

        Mock::given(method("GET"))
            .and(path(&mock_path))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let pokemon_url = format!("{}{}", &mock_server.uri(), &mock_path);
        let pokemon_result = fetch_pokemon(&pokemon_url).await;

        assert_eq!(true, pokemon_result.is_err());

        assert_eq!(WrapperError::ParsingError, pokemon_result.err().unwrap());

        Ok(())
    }
}
