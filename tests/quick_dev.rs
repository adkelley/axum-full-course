use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Mike").await?.print().await?;

    // We won't use this so much, but it's here for reference
    // hc.do_get("/src/main.rs").await?.print().await?;
    Ok(())
}
