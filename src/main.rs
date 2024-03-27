#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await
        .unwrap();
}
