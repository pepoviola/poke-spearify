use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

#[derive(Deserialize, Serialize)]
struct PokemonResponse {
    name: String,
    description: String,
}

pub async fn get(req: Request<()>) -> tide::Result {
    let pokemon_name = req.param("pokemon_name")?;
    let pokemon = PokemonResponse {
        name: pokemon_name.to_string(),
        description: "some".into(),
    };

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&pokemon)?);
    Ok(res)
}
