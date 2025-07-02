use std::fs;

use comrak::plugins::syntect::SyntectAdapterBuilder;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{http::Status, State};
use rocket_dyn_templates::{context, tera::Tera, Template};

use crate::{blog::{QueryPost, QueryTag}, SiteState};



#[get("/<blog>")]
pub(crate) fn blog_post_page(blog: &str, state: &State<SiteState>) -> Result<Template, Status> {
    let mut connection = match state.pool.get() {
        Err(err) => {
            error!("Error getting connection from pool: {:?}", err);
            return Err(Status::InternalServerError);
        },
        Ok(pool) => pool,
    };
    use crate::schema::{blog_posts::dsl::*, blog_tags::dsl::*};
    
    let post: QueryPost = match blog_posts.filter(name.eq(blog)).select(QueryPost::as_select()).first(&mut connection) {
        Ok(post) => post,
        Err(err) => { 
            error!("Error accessing blog post: {:?}", err);
            return Err(Status::NotFound);
        }
    };
    let tags: Vec<QueryTag> = match blog_tags.filter(project.eq(post.id)).select(QueryTag::as_select()).get_results(&mut connection) {
        Ok(tags) => tags,
        Err(err) => { 
            error!("Error accessing blog tags: {:?}", err);
            return Err(Status::InternalServerError);
        }
    };
    let post_content = match fs::read_to_string(post.content_path.clone()) {
        Err(err) => {
            error!("Error reading content file: {:?}", err);
            return Err(Status::NotFound);
        }
        Ok(markdown) => {
            let mut plugins = comrak::Plugins::default();
           let builder = SyntectAdapterBuilder::new().theme("base16-ocean.dark");
            let adapter = builder.build();
            plugins.render.codefence_syntax_highlighter = Some(&adapter);
            comrak::markdown_to_html_with_plugins(&markdown, &comrak::Options::default(), &plugins)
        },
    };
    
    Ok(Template::render("blog_post", context!{post, post_content, tags}))
}
#[get("/?<order>&<tags>&<project_done>&<blog_done>")]
pub(crate) fn all_blog_posts(state: &State<SiteState>,
    order: Option<&str>,
    tags: Option<Vec<&str>>, 
    project_done: Option<bool>,
    blog_done: Option<bool>) -> Result<Template, Status> {
    let mut connection = match state.pool.get() {
        Err(err) => {
            error!("Error getting connection from pool: {:?}", err);
            return Err(Status::InternalServerError);
        },
        Ok(pool) => pool,
    };
    
    let order = order.unwrap_or("desc");
    let project_done = project_done.unwrap_or(true);
    let blog_done = blog_done.unwrap_or(true);

    use crate::schema::blog_posts::dsl::*;
    let x = blog_posts.select(QueryPost::as_select());
    let posts = match order {
        "asc" => x.order_by(modified.asc()).get_results(&mut connection),
        "desc" => x.order_by(modified.desc()).get_results(&mut connection),
        _ => return Err(Status::UnprocessableEntity),
    };

    let Ok(posts) = posts else { return Err(Status::InternalServerError)};
    // .;

   debug!("Order: {:?} | Tags: {:?} | Project_finished: {:?} | Blog_Finished: {:?}", order, tags, project_done, blog_done);
   Ok(Template::render("all_blog_posts", context!{posts}))
} 
