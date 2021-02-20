use tide::{Request, Response};

pub async fn health_check(_req: Request<()>) -> tide::Result {
    Ok(Response::new(200))
}