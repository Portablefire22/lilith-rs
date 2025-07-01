use std::env;

use rocket::fs::{relative, FileServer, Options};
use rocket_dyn_templates::{context, Template};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{})
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
        .register("/", catchers![])
        .launch()
        .await?;
    Ok(())
}
