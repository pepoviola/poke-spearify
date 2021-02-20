use tide::Server;

use crate::controllers::health_check;
use crate::controllers::pokemon;

pub async fn build() -> Server<()> {

    let mut app = tide::new();

    // health check
    app.at("/health_check").get(health_check::health_check);

    // pokemon api route
    app.at("/pokemon/:pokemon_name").get(pokemon::get);

    app
}