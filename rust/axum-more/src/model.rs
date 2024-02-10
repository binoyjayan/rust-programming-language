//! (mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketPayload {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    store: MockStore,
}

impl ModelController {
    pub fn new() -> Self {
        ModelController {
            store: MockStore::new(),
        }
    }

    pub async fn create_ticket(&self, payload: TicketPayload) -> Result<Ticket> {
        self.store.create_ticket(payload).await
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        self.store.list_tickets().await
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        self.store.delete_ticket(id).await
    }
}

#[derive(Clone)]
pub struct MockStore {
    tickets: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl MockStore {
    pub fn new() -> Self {
        MockStore {
            tickets: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn create_ticket(&self, payload: TicketPayload) -> Result<Ticket> {
        let mut tickets = self.tickets.lock().unwrap();
        let id = tickets.len() as u64 + 1;
        let ticket = Ticket {
            id,
            title: payload.title,
        };
        tickets.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let tickets = self.tickets.lock().unwrap();
        // Filter out the None values
        let tickets = tickets.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut tickets = self.tickets.lock().unwrap();
        let ticket = tickets.get_mut(id as usize - 1).and_then(|t| t.take());
        ticket.ok_or(Error::NotFound)
    }
}
