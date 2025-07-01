use std::{env, path::PathBuf};

use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};
use rocket::{fs::{relative, FileServer, Options}, http::Status, Request};
use rocket_dyn_templates::{context, Template};

use crate::{contact::{ContactError, ContactInformation}, database::{get_connection_pool, init_post_db}};

#[macro_use] extern crate rocket;

mod contact;
mod database;
mod project;
mod blog;
mod schema;

#[get("/")]
fn index() -> Result<Template, Status> {
    Ok(Template::render("index", context!{}))
}

#[get("/contact")]
fn contact_page() -> Result<Template, Status> {
    let contacts = match ContactInformation::from_file(PathBuf::from("public/yaml/contact.yaml")) {
        Ok(contacts) => contacts,
        Err(ContactError::Io(err)) => {
            error!("Could not open contacts: {:?}", err);
            return Err(Status::NotFound)
        },
        Err(ContactError::Serde(err)) => {
            error!("Could not open contacts: {:?}", err);
            return Err(Status::InternalServerError)
        },
    };
    Ok(Template::render("contact", context!{contacts}))
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

pub(crate) struct SiteState {
    pub(crate) pool: Pool<ConnectionManager<SqliteConnection>>
}

#[main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().expect("Could not load .env file");
    if let Ok(path) = env::var("LOG_CONFIG_PATH") {
        log4rs::init_file(path, Default::default()).unwrap();
    } else {
        panic!("Could not initialise logging, LOG_CONFIG_PATH variable not set correctly!")
    }
    
    let pool = get_connection_pool().unwrap();
    init_post_db(&pool).unwrap();
    
    info!("Launching Website!");

    let _rocket = rocket::build()
        .attach(Template::fairing())
        .manage(SiteState{pool: pool})
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        .mount("/", routes![index, contact_page])
        .mount("/projects", routes![project::route::all_projects, project::route::project_page])
        .mount("/blog", routes![blog::route::all_blog_posts, blog::route::blog_post_page])
        .register("/", catchers![not_found, internal_error, general_error])
        .launch()
        .await?;
    Ok(())
}
