#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::response::content::Json;

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json(
        "{
    'status': 'success',
    'message': 'Hello API!'
  }",
    )
}

fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/api", routes![hello]).launch();
}
