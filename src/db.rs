use mongodb::db::Database;
use mongodb::{Client, ThreadedClient};
use std::env;

/// create_mongo_connection creates a connection to a
/// mongodb server. Returns a database.
///
/// # Arguments
/// none
///
/// # Examples
///
/// ```
/// use::db::create_mongo_connection;
/// let database = create_mongo_connection();
/// ```
pub fn create_mongo_connection() -> Database {
    let host = env::var("MONGO_HOST").expect("MONGO_HOST not set");
    let port: u16 = env::var("MONGO_PORT")
        .expect("MONGO_PORT not set")
        .parse()
        .expect("Invalid string conversion to u16");
    let database: String = env::var("MONGO_DATABASE").expect("MONGO_DATABASE not set");

    let client = match Client::connect(&host, port) {
        Ok(client) => client,
        Err(err) => panic!("Client unable to connect to mongo server, {:?}", err),
    };

    client.db(&database)
}
