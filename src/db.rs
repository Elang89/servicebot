use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use mongodb::{Client, ThreadedClient};

pub fn create_mongo_connection((host, port): (&str, u16)) -> Client {
    Client::connect(host, port).expect("Client failed to connect")
}
