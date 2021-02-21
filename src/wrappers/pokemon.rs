use serde::Deserialize;

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

impl Pokemon {
    fn get_description(&self) -> Result<String, tide::Error> {
        let description = self
            .flavor_text_entries
            .iter()
            .find(|desc| desc.language.name == "en")
            .ok_or_else(|| tide::Error::from_str(500, "Unexpected Error".to_string()))?;

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

pub async fn get_description(pokemon_name: &str) -> Result<String, tide::Error> {
    let pokemon_url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", pokemon_name);
    let pokemon = fetch_pokemon(&pokemon_url).await?;
    let description = pokemon.get_description()?;
    Ok(description)
}

async fn fetch_pokemon(pokemon_url: &str) -> Result<Pokemon, tide::Error> {
    let mut res = surf::get(pokemon_url).await?;

    let status: u16 = res.status().into();
    match status {
        200 => {
            let pokemon: Pokemon = res.body_json().await.map_err(|e|{
                tide::log::error!("Error: {}, deserializing response to Pokemon", e);
                tide::Error::from_str(500, "Unexpected Error".to_string())
            })?;
            Ok(pokemon)
        }
        404 => Err(tide::Error::from_str(404, "Not Found".to_string())),
        _ => Err(tide::Error::from_str(500, "Unexpected Error".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const POKEMON_PATH: &str = "/api/v2/pokemon-species/";

    #[async_std::test]
    async fn fetch_pokemon_description() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;
        let existing_pokemon = "charizard";
        let mock_path = format!("{}{}", POKEMON_PATH, existing_pokemon.to_string());

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
        let mock_path = format!("{}{}", POKEMON_PATH, non_existing_pokemon.to_string());

        Mock::given(method("GET"))
            .and(path(&mock_path))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let pokemon_url = format!("{}{}", &mock_server.uri(), &mock_path);
        let pokemon = fetch_pokemon(&pokemon_url).await;

        assert_eq!(true, pokemon.is_err());

        let status_code = pokemon.err().unwrap().status();
        assert_eq!(tide::http::StatusCode::NotFound, status_code);

        Ok(())
    }

    #[async_std::test]
    async fn fetch_pokemon_without_description() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;
        let existing_pokemon = "charizard";
        let mock_path = format!("{}{}", POKEMON_PATH, existing_pokemon.to_string());

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

        let status_code = description.err().unwrap().status();
        assert_eq!(tide::http::StatusCode::InternalServerError, status_code);

        Ok(())
    }

    #[async_std::test]
    async fn fetch_pokemon_parse_error() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;
        let existing_pokemon = "charizard";
        let mock_path = format!("{}{}", POKEMON_PATH, existing_pokemon.to_string());

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

        let status_code = pokemon_result.err().unwrap().status();
        assert_eq!(tide::http::StatusCode::InternalServerError, status_code);

        Ok(())
    }
}
