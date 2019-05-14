use crate::errors::ServiceError;
use crate::model::PushEvent;

use actix::{Actor, Context, Handler, Message};
use serde_derive::Deserialize;
use telegram_bot_fork::{Api, CanSendMessage, ChatId};

/// TbActor struct
pub struct TbActor {
    pub client: Api,
}

#[derive(Debug, Deserialize)]
pub struct PushEventMsg {
    pub reference: String,
}

impl Message for PushEventMsg {
    type Result = ();
}

impl TbActor {
    /// new creates a new client and returns
    /// a TbActor
    ///
    pub fn new(client: Api) -> TbActor {
        TbActor { client: client }
    }
}

impl Actor for TbActor {
    type Context = Context<Self>;
}

impl Handler<PushEventMsg> for TbActor {
    type Result = ();

    /// handle receives a PushEvent as a message and then
    /// sends it through telegram
    fn handle(&mut self, msg: PushEventMsg, _: &mut Self::Context) {
        let chat = ChatId::new(742515568);
        self.client.spawn(chat.text(msg.reference));
    }
}
