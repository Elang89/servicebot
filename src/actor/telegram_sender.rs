use actix::{Actor, Context, Handler, Message};
use serde_derive::Deserialize;
use telegram_bot_fork::{Api, CanSendMessage, ChatId};

/// TbActor struct
pub struct TbSenderActor(pub Api);

#[derive(Debug, Deserialize)]
pub struct PushEventMsg {
    pub reference: String,
}

impl Message for PushEventMsg {
    type Result = ();
}

impl Actor for TbSenderActor {
    type Context = Context<Self>;
}

impl Handler<PushEventMsg> for TbSenderActor {
    type Result = ();

    /// handle receives a PushEvent as a message and then
    /// sends it through telegram
    fn handle(&mut self, msg: PushEventMsg, _: &mut Self::Context) {
        println!("Message: {:?}", msg);
        // let chat = ChatId::new(742515568);
        // self.client.spawn(chat.text(msg.reference));
    }
}
