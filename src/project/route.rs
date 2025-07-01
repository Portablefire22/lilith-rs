use rocket::{http::Status, State};
use rocket_dyn_templates::{context, Template};

use crate::SiteState;


#[get("/<project>")]
pub(crate) fn project_page(project: &str, state: &State<SiteState>) -> Result<Template, Status> {
   let mut connection = match state.pool.get() {
       Err(err) => {
           error!("Error getting connection from pool: {:?}", err);
           return Err(Status::InternalServerError);
       },
       Ok(pool) => pool,
   };
   Ok(Template::render("project", context!{}))
}

#[get("/")]
pub(crate) fn all_projects(state: &State<SiteState>) -> Result<Template, Status> {
   let mut connection = match state.pool.get() {
       Err(err) => {
           error!("Error getting connection from pool: {:?}", err);
           return Err(Status::InternalServerError);
       },
       Ok(pool) => pool,
   };
   Ok(Template::render("all_projects", context!{}))
} 
