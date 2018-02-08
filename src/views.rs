
use rocket;

use rocket::response::Redirect;
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;

use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::Template;


#[catch(404)]
fn not_found(req: &Request) -> Template {

    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
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

fn rocket()->rocket::Rocket {

    rocket::ignite()
        .mount("/", routes![hello])
        .attach(Template::fairing())
        .catch(catchers![not_found])
}

pub fn server() {

    let r =rocket();
    r.launch();
}