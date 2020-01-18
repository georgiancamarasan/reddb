use dotenv::dotenv;
use iron::prelude::*;
use router::Router;

use dotenv_codegen::dotenv;
mod dstore;
mod handlers;
use dstore::DStore;
#[macro_use]
extern crate quick_error;

fn main() {
    dotenv().ok();
    let mut db = DStore::new(dotenv!("DB_PATH")).unwrap();
    let doc = r#"
        {
            "id": 1,
            "data": {
            }
        }"#;
    let data = doc.to_string();
    let doc = db.insert(data);
    let _id = &doc["_id"];
    let result = db.find_by_id(&_id);
    //db.insert(doc);
    db.insert(r#"{"name":"notsaved1"}"#.to_string());
    db.insert(r#"{"name":"notsaved2", "leches": "34"}"#.to_string());
    let result2 = db.find(r#"{"name":"john"}"#.to_string());
    //println!("FIND {:?}", result2);
    // db.put(r#"{"id": 1,"data": {}}"#.to_string());
    // db.put(r#"{"id": 1,"data": {}}"#.to_string());
    // db.put(r#"{"id": 1,"data": {}}"#.to_string());
    //db.get();
    db.persist();
    //db.put("key".to_string(), "value".to_string()).persist();
    //println!("DATA {:?}", data);
    let mut router = Router::new();
    for handler in handlers::get_handlers() {
        match handler.method {
            handlers::Method::Get => {
                println!("Setting up GET method {}", handler.route);
                router.get(&handler.route, handler.handler, &handler.route);
            }
            handlers::Method::Post => {
                println!("Setting up POST method {}", handler.route);
                router.post(&handler.route, handler.handler, &handler.route);
            }
        }
    }
    let host_addr = dotenv!("HOST_ADDRESS");

    println!("Server up on http://{}", &host_addr);
    Iron::new(router).http(&host_addr).unwrap();
}
