use mongodb::db::{Database, ThreadedDatabase};
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
    let user: String = env::var("MONGO_USER").expect("MONGO_USER not set");
    let password: String = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD not set");
    let database: String = env::var("MONGO_DATABASE").expect("MONGO_DATABASE not set");

    let client = match Client::connect(&host, port) {
        Ok(client) => client,
        Err(err) => panic!("Client unable to connect to mongo server, {:?}", err),
    };

    let db = client.db(&database);

    match db.auth(&user, &password) {
        Ok(_) => db,
        Err(err) => panic!("Authentication failed, {:?}", err),
    }
}
