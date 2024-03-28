#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{config::TlsConfig, get, routes, Config};
use std::{
    net::Ipv6Addr,
    process::{Command, Output},
    str::FromStr,
};

#[get("/favicon.ico")]
fn favicon() -> &'static [u8] {
    include_bytes!("../favicon.ico")
}

#[get("/apple-touch-icon.png")]
fn pngfavicon() -> &'static [u8] {
    include_bytes!("../favicon.png")
}

#[get("/")]
fn index() -> String {
    match Command::new("/usr/games/fortune").arg("-a").output() {
        Ok(Output {
            status: _,
            stdout,
            stderr: _,
        }) => stdout.iter().map(|byte| *byte as char).collect(),
        Err(e) => format!("No fortune today ({e})."),
    }
}

#[rocket::main]
async fn main() {
    rocket::custom(Config {
        address: std::net::IpAddr::V6(Ipv6Addr::from_str("::").unwrap()),
        port: 443,
        workers: 1,
        tls: Some(TlsConfig::from_bytes(
            include_bytes!("../../Downloads/radiocrouton.fr_ssl_certificate.cer"),
            include_bytes!("../../radiocrouton.fr_private_key.key"),
        )),
        ..Default::default()
    })
    .mount("/", routes![index, favicon, pngfavicon])
    .launch()
    .await
    .unwrap();
}
