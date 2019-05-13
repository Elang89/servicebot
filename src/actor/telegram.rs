use crate::model::PushEvent;

use actix::{Actor, Context, Handler};
use telegram_bot_fork::{Api, CanSendMessage, ChatId};

/// TbActor struct
pub struct TbActor {
    pub client: Api,
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

impl Handler<PushEvent> for TbActor {
    type Result = ();

    /// handle receives a PushEvent as a message and then
    /// sends it through telegram
    fn handle(&mut self, msg: PushEvent, _: &mut Self::Context) {
        let chat = ChatId::new(742515568);
        self.client.spawn(chat.text(msg.reference));
    }
}
