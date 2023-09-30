use actix::prelude::*;
use actix::Actor;
use pyo3::prelude::*;
use uuid::Uuid;

use std::collections::HashMap;

use crate::websockets::MyWs;

#[derive(Default)]
#[pyclass]
pub struct WebSocketRegistry {
    // A map of client IDs to their Actor addresses.
    clients: HashMap<Uuid, Addr<MyWs>>,
}

impl actix::Supervised for WebSocketRegistry {}

impl SystemService for WebSocketRegistry {}

impl Actor for WebSocketRegistry {
    type Context = Context<Self>;
}

pub struct Register {
    pub id: Uuid,
    pub addr: Addr<MyWs>,
}

impl Message for Register {
    type Result = ();
}

impl Handler<Register> for WebSocketRegistry {
    type Result = ();

    fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) {
        dbg!("Registering client {}", msg.id);
        dbg!("Clients: {:?}", &self.clients);
        self.clients.insert(msg.id, msg.addr);
    }
}

// New message for sending text to a specific client
pub struct SendText {
    pub recipient_id: Option<Uuid>,
    pub message: String,
    pub sender_id: Uuid,
}

impl Message for SendText {
    type Result = ();
}

impl WebSocketRegistry {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn start() -> Addr<Self> {
        Self::from_registry()
    }
}

impl Handler<SendText> for WebSocketRegistry {
    type Result = ();

    fn handle(&mut self, msg: SendText, _ctx: &mut Self::Context) {
        let recipient_id = msg.recipient_id.unwrap();

        if let Some(client_addr) = self.clients.get(&recipient_id) {
            client_addr.do_send(msg);
        }
    }
}

pub struct SendMessageToAll {
    pub message: String,
    pub sender_id: Uuid,
}

impl Message for SendMessageToAll {
    type Result = ();
}

impl Handler<SendMessageToAll> for WebSocketRegistry {
    type Result = ();

    fn handle(&mut self, msg: SendMessageToAll, _ctx: &mut Self::Context) {
        dbg!("Sending message to client {}", self.clients.len());
        for (id, client) in &self.clients {
            client.do_send(SendText {
                recipient_id: None,
                message: msg.message.clone(),
                sender_id: msg.sender_id.clone(),
            });
        }
    }
}
