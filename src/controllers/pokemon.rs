use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

use crate::wrappers;

#[derive(Deserialize, Serialize)]
struct PokemonResponse {
    name: String,
    description: String,
}

pub async fn get(req: Request<()>) -> tide::Result {
    let pokemon_name = req.param("pokemon_name")?;

    let description = wrappers::pokemon::get_description(pokemon_name).await?;
    let translated_description = wrappers::shakespeare::get_translation(&description).await?;

    let pokemon = PokemonResponse {
        name: pokemon_name.to_string(),
        description: translated_description,
    };

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&pokemon)?);
    Ok(res)
}
