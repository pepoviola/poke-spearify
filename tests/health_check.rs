use poke_spearify::server;

#[async_std::test]
async fn health_check() -> tide::Result<()> {
    dotenv::dotenv().ok();
    let app = server::build().await;

    let res = surf::Client::with_http_client(app)
        .get("https://example.com/health_check")
        .await?;

    assert_eq!(200, res.status());
    Ok(())
}
