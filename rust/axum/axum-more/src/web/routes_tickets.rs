use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketPayload};
use crate::Result;

use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    mc: State<ModelController>,
    ctx: Ctx,
    Json(payload): Json<TicketPayload>,
) -> Result<Json<Ticket>> {
    println!("-->> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ctx, payload).await?;
    Ok(Json(ticket))
}

async fn list_tickets(mc: State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("-->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    mc: State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("-->> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}
