use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

use crate::server::State;

#[derive(Deserialize, Serialize)]
pub struct PokemonResponse {
    name: String,
    description: String,
}

pub async fn get(req: Request<State>) -> tide::Result {
    let pokemon_name = req.param("pokemon_name")?;

    let pokemon_wrapper = req.state().pokemon_wrapper.clone();
    let description = pokemon_wrapper.get_description(pokemon_name).await?;
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
