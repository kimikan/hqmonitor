
use rocket;

use rocket::response::Redirect;
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;

use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::Template;


#[error(404)]
fn not_found(req: &Request) -> Template {

    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

use serde::Serialize;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>
}

#[derive(FromForm)]
struct Person {
    name: String,
    age: Option<u8>
}

#[get("/hello?<person>")]
fn hello(person: Person) -> String {
    if let Some(age) = person.age {
        format!("Hello, {} year old named {}!", age, person.name)
    } else {
        format!("Hello {}!", person.name)
    }
}

#[get("/")]
fn index() -> Template {

    let mut v:Vec<String> = vec![];

    let today = ::ALARM_MANAGER.get_today();
    let logs = ::ALARM_MANAGER.get_by_date(today);

    println!("xxxxxxxxxxxxx today:{}, {:?}", today, logs);

    if let Some(ls) = logs {
        let x = ls.read().unwrap();

        for l in &(*x) {
            println!("{}", l);
            v.push(l.clone());

        }

    }

    let context = TemplateContext { name : "logs".to_string(), items: v };
    Template::render("list", context)
}

fn rocket()->rocket::Rocket {

    rocket::ignite()
        .mount("/", routes![index, hello])
        .attach(Template::fairing())
        .catch(errors![not_found])
}

pub fn server() {

    let r =rocket();
    r.launch();
}