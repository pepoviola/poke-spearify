use crate::server::State;
use tide::{Request, Response};

pub async fn health_check(_req: Request<State>) -> tide::Result {
    Ok(Response::new(200))
}
