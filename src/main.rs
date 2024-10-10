#[macro_use]
extern crate rocket;

use duckdb::Connection;
use rocket::serde::json::Json;
use rocket::State;

#[derive(serde::Deserialize)]
struct DocumentBase {
    name: String,
    content: String,
}

#[derive(serde::Deserialize)]
struct DocumentCreate {
    base: DocumentBase,
}

struct Document {
    document_id: String,
    base: DocumentBase,
    content_md5: String,
    created_at: i64,
}

// Define DbConnection to be thread-safe
struct DbConnection {
    connection: std::sync::Mutex<Connection>, // Use Mutex for thread safety
}

#[get("/")]
fn index() -> &'static str {
    "OK"
}

#[post("/documents", data = "<document>")]
fn documents(db: &State<DbConnection>, document: Json<DocumentCreate>) -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    let db_connection = DbConnection {
        connection: std::sync::Mutex::new(Connection::open_in_memory().unwrap()),
    };
    rocket::build()
        .manage(db_connection)
        .mount("/", routes![index, documents])
}
