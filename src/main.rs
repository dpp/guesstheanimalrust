#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket::http::RawStr;
use rocket_contrib::json::{/*Json,*/ JsonValue};

// use rocket::response::content::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/j/<name>")]
fn jhello(name: String) -> Option<JsonValue> {
    if name.contains("Cat") {
        None
    } else {
   Some( json!({"name": name}))
    }
}

fn main() {
    println!("Hello, world! I like cats!!");
    rocket::ignite().mount("/", routes![index,jhello, hello]).launch();
}

