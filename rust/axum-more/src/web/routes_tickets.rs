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
    Json(payload): Json<TicketPayload>,
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(payload).await?;
    Ok(Json(ticket))
}

async fn list_tickets(mc: State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(mc: State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
