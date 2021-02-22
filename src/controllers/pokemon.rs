use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

use crate::server::State;
use crate::wrappers::errors::WrapperError;

#[derive(Deserialize, Serialize)]
pub struct PokemonResponse {
    name: String,
    description: String,
}

pub async fn get(req: Request<State>) -> tide::Result {
    let pokemon_name = req.param("pokemon_name")?;

    let pokemon_wrapper = req.state().pokemon_wrapper.clone();
    let description = pokemon_wrapper
        .get_description(pokemon_name)
        .await
        .map_err(|e| match e {
            WrapperError::NotFound => tide::Error::from_str(404, "Not Found".to_string()),
            WrapperError::TooManyRequests => {
                tide::Error::from_str(429, "Too Many Requests".to_string())
            }
            _ => tide::Error::from_str(500, "Unexpected Error".to_string()),
        })?;
    let shakespeare_wrapper = req.state().shakespeare_wrapper.clone();
    let translated_description = shakespeare_wrapper.get_translation(&description).await?;

    let pokemon = PokemonResponse {
        name: pokemon_name.to_string(),
        description: translated_description,
    };

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&pokemon)?);
    Ok(res)
}
