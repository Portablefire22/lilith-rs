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

#[get("/?<order>&<tags>&<project_finished>&<blog_finished>")]
pub(crate) fn all_projects(
    order: Option<&str>,
    tags: Option<Vec<&str>>, 
    project_finished: bool,
    blog_finished: bool,
    state: &State<SiteState>) -> Result<Template, Status> {
   let mut connection = match state.pool.get() {
       Err(err) => {
           error!("Error getting connection from pool: {:?}", err);
           return Err(Status::InternalServerError);
       },
       Ok(pool) => pool,
   };
   debug!("Order: {:?} | Tags: {:?} | Project_finished: {:?} | Blog_Finished: {:?}", order, tags, project_finished, blog_finished);
   Ok(Template::render("all_projects", context!{}))
} 
