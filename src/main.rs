#![feature(proc_macro_hygiene, decl_macro)]

use std::{net::Ipv6Addr, str::FromStr};

use rocket::{config::TlsConfig, Config};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::custom(Config {
        address: std::net::IpAddr::V6(Ipv6Addr::from_str("::").unwrap()),
        port: 443,
        workers: 1,
        tls: Some(TlsConfig::from_bytes(
            "../../Downloads/radiocrouton.fr_ssl_certificate.cer",
            "../../radiocrouton.fr_private_key.key",
        )),
        ..Default::default()
    })
    .mount("/", routes![index])
    .launch()
    .await
    .unwrap();
}
