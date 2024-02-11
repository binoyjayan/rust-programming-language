use anyhow::Result;
use serde_json::json;

// cargo test -- --nocapture

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello2/Binoy").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );
    req_login.await?.print().await?;

    let req_ticket_create = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket 1",
        }),
    );
    req_ticket_create.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
