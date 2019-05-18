use actix::{Actor, Context, StreamHandler};
use telegram_bot_fork::{Api, Error, Update};

pub struct TbReceiverActor(pub Api);

impl StreamHandler<Update, Error> for TbReceiverActor {
    fn handle(&mut self, item: Update, ctx: &mut Context<TbReceiverActor>) {
        println!("item: {:?}", item);
    }
}

impl Actor for TbReceiverActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let stream = self.0.stream();
        Self::add_stream(stream, ctx);
    }
}
