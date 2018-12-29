#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
use rocket::http::RawStr;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;
use std::sync::Mutex;

type ID = usize;

#[derive(Debug, Serialize, Deserialize)]
enum AnimalInfo {
    Question {
        question: String,
        yes_answer: u32,
        no_answer: u32,
    },
    Answer {
        answer: String,
    },
}

type AnimalQuestions = HashMap<ID, AnimalInfo>; 
type MessageMap = Mutex<AnimalQuestions>;

fn list_animals(info: &AnimalQuestions) -> Vec<String> {
    info.values()
        .flat_map(|x| match x {
            AnimalInfo::Answer { answer: z } => Some(z.clone()).into_iter(),
            _ => None.into_iter(),
        })
        .collect()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    return format!("Yo Hello, {}!", name.as_str());
}

#[get("/j/<name>")]
fn jhello(name: String) -> Option<JsonValue> {
    if name.contains("Cat") {
        None
    } else {
        Some(json!({"name": name, "age": 33}))
    }
}

fn main() {
    let mut baseline = HashMap::<ID, AnimalInfo>::new();

    baseline.insert(
        1,
        AnimalInfo::Question {
            question: String::from("Does it swim?"),
            yes_answer: 2,
            no_answer: 3,
        },
    );

    baseline.insert(
        2,
        AnimalInfo::Answer {
            answer: String::from("Fish"),
        },
    );

    baseline.insert(
        3,
        AnimalInfo::Answer {
            answer: String::from("Bird"),
        },
    );

    println!("Hello, world! I like cats!!");
    rocket::ignite()
        .mount("/", routes![index, jhello, hello])
        .manage(Mutex::new(baseline))
        .launch();
}
