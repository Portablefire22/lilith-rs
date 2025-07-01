use std::env;

use rocket::{fs::{relative, FileServer, Options}, http::Status, Request};
use rocket_dyn_templates::{context, Template};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Result<Template, Status> {
    Ok(Template::render("index", context!{}))
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("not_found", context!{})
}

#[catch(500)]
fn internal_error(status: Status, req: &Request) -> Template {
    error!("Client@{:?} encountered an Internal Server Error doing {} {}", req.client_ip(), req.method(), req.uri());
    let message = "An internal server error has occured.";
    Template::render("general_error", context!{status, message})
}

#[catch(default)]
fn general_error(status: Status, req: &Request) -> Template {
    error!("Client@{:?} encountered a Status {} doing {} {}", req.client_ip(), status.code, req.method(), req.uri());
    let message = status.reason_lossy();
    Template::render("general_error", context!{status, message})
}

#[main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().expect("Could not load .env file");
    if let Ok(path) = env::var("LOG_CONFIG_PATH") {
        log4rs::init_file(path, Default::default()).unwrap();
    } else {
        panic!("Could not initialise logging, LOG_CONFIG_PATH variable not set correctly!")
    }
    info!("Launching Website!");

    let _rocket = rocket::build()
        .attach(Template::fairing())
        // .manage(SiteState{pool: pool})
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        .mount("/", routes![index])
        .register("/", catchers![not_found, internal_error, general_error])
        .launch()
        .await?;
    Ok(())
}
