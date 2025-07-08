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
            let mut options = comrak::Options::default();
            options.extension.header_ids = Some("".to_string());
            options.extension.table = true;
            comrak::markdown_to_html_with_plugins(&markdown, &options, &plugins)
        },
    };
    
    Ok(Template::render("blog_post", context!{post, post_content, tags}))
}
#[get("/?<order>&<tags>&<project_done>&<show_unfinished>")]
pub(crate) fn all_blog_posts(state: &State<SiteState>,
    order: Option<&str>,
    tags: Option<Vec<&str>>, 
    project_done: Option<bool>,
    show_unfinished: Option<bool>) -> Result<Template, Status> {
    let mut connection = match state.pool.get() {
        Err(err) => {
            error!("Error getting connection from pool: {:?}", err);
            return Err(Status::InternalServerError);
        },
        Ok(pool) => pool,
    };
    
    let order = order.unwrap_or("desc");
    let show_unfinished = show_unfinished.unwrap_or(false);

    use crate::schema::blog_posts::dsl::*;
    // let mut x = blog_posts.select(QueryPost::as_select()).filter(project_finished.eq(project_done)).filter(blog_finished.eq(!show_unfinished));

    let mut query = blog_posts.into_boxed().select(QueryPost::as_select());
    
    query = match order {
        "asc" => query.order_by(modified.asc()),
        "desc" => query.order_by(modified.desc()),
        _ => return Err(Status::UnprocessableEntity),
    };

    // Include unfinished in the search?
    if !show_unfinished {
        query = query.filter(blog_finished.eq(true))
    };
    
    if let Some(project_done) = project_done {
        query = query.filter(project_finished.eq(project_done));
    }

    let posts: Vec<QueryPost> = match query.load(&mut connection) {
        Ok(posts) => posts,
        Err(err) => {
            error!("Error accssing all blog posts: {:?}", err);
            return Err(Status::InternalServerError);
        }
    };
    debug!("Order: {:?} | Tags: {:?} | Project_finished: {:?} | Blog_Finished: {:?}", order, tags, project_done, show_unfinished);
    Ok(Template::render("all_blog_posts", context!{posts}))
} 
