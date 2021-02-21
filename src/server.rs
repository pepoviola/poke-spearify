use tide::Server;

use crate::controllers::health_check;
use crate::controllers::pokemon;
use crate::wrappers::pokemon::PokemonWrapper;
use crate::wrappers::shakespeare::ShakespeareWrapper;

#[derive(Clone, Debug)]
pub struct State {
    pub shakespeare_wrapper: ShakespeareWrapper,
    pub pokemon_wrapper: PokemonWrapper,
}

pub async fn build(
    shakespeare_wrapper: ShakespeareWrapper,
    pokemon_wrapper: PokemonWrapper,
) -> Server<State> {
    let state = State {
        shakespeare_wrapper,
        pokemon_wrapper,
    };

    let mut app = tide::with_state(state);

    // health check
    app.at("/health_check").get(health_check::health_check);

    // pokemon api route
    app.at("/pokemon/:pokemon_name").get(pokemon::get);

    app
}
