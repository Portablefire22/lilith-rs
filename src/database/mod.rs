use std::{env::{self, VarError}, fs, path::PathBuf, time::{Instant, UNIX_EPOCH}};

use chrono::DateTime;
use diesel::{delete, insert_into, r2d2::{ConnectionManager, Pool, PoolError, PooledConnection}, sql_query, RunQueryDsl, SqliteConnection};

use crate::blog::{InsertPost, Post};

#[derive(Debug)]
pub(crate) enum DbError {
    EnvironmentVariableError(VarError),
    DatabaseConnectionError(PoolError),
    SerdeError(serde_yaml::Error),
    IoError(std::io::Error)
}

pub(crate) fn get_connection_pool() -> Result<Pool<ConnectionManager<SqliteConnection>>, DbError> {
    let db_path = match env::var("DATABASE_URL"){
        Ok(path) => path,
        Err(err) => {
            error!("DATABASE_URL variable error: {:?}", err);
            return Err(DbError::EnvironmentVariableError(err))
        }
    };
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);

    // Kinda gives me AIDs 
    match Pool::builder()
        .test_on_check_out(true)
        .build(manager) {
            Ok(pool) => Ok(pool),
            Err(err) => Err(DbError::DatabaseConnectionError(err))
        }
}

// It's a lot easier to just wipe the DB instead of trying to edit things in place.
// None of this data is persistent so we don't need to keep anything between launches.
// Seemed to work fine on the mdrs project.
pub(crate) fn clear_db(pool: &Pool<ConnectionManager<SqliteConnection>>) {
    use crate::schema::blog_posts::dsl::*;
    use crate::schema::blog_tags::dsl::*;
    info!("Clearing DB...");
    let mut connection = pool.get().unwrap();
    delete(blog_posts).execute(&mut connection).unwrap();
    delete(blog_tags).execute(&mut connection).unwrap();
    sql_query("DELETE FROM sqlite_sequence where name='projects'").execute(&mut connection).unwrap();
    info!("Cleared DB");
}

pub(crate) fn add_blog_file_to_db(post_path: &PathBuf, connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>) -> Result<(), DbError> {
    use crate::schema::blog_posts::dsl::blog_posts;
    let yaml = match fs::read_to_string(&post_path) { 
        Err(err) => return Err(DbError::IoError(err)),
        Ok(yaml) => yaml,
    };
    let post: Post = match serde_yaml::from_str(&yaml) {
        Err(err) => return Err(DbError::SerdeError(err)),
        Ok(post) => post,
    };
    
    let modified = post_path.metadata().unwrap().modified().unwrap();
    let modified = modified.duration_since(UNIX_EPOCH).unwrap();
    let modified = DateTime::from_timestamp_millis(modified.as_millis() as i64).unwrap().naive_utc();
    
    let parent_dir = match post_path.parent() {
        None => post_path,
        Some(dir) => dir,
    };

    let tmp_post = InsertPost {
        name: post.name.clone(),
        description: post.description,
        image: post.image,
        content_path: format!("{}/{}", parent_dir.to_str().unwrap(), post.content_path), // WHY
        blog_finished: post.blog_finished,
        project_finished: post.project_finished,
        hiatus_since: post.hiatus_since,
        modified,
    };
    info!("Added project: {}", post.name);
    let _ = insert_into(blog_posts)
        .values(tmp_post)
        .execute(connection).unwrap();
    Ok(())
}

pub(crate) fn init_post_db(pool: &Pool<ConnectionManager<SqliteConnection>>) -> Result<(), DbError> {
    info!("Intitialising Post DB...");
    let start = Instant::now();
    clear_db(pool);
    let mut connection = pool.get().unwrap();
    let top_level_dir = match fs::read_dir("posts/") {
        Err(err) => return Err(DbError::IoError(err)),
        Ok(ok) => ok,
    };
    for top_path in top_level_dir {
        debug!("{:?}", top_path);
        if let Ok(top_path) = top_path {
            if top_path.path().is_dir() {
                let dir = match fs::read_dir(top_path.path()) {
                    Err(err) => return Err(DbError::IoError(err)),
                    Ok(ok) => ok,
                };
                for path in dir {
                    if let Ok(path) = path {
                        match add_blog_file_to_db(&path.path(), &mut connection) {
                            Err(err) => (),
                            Ok(_) => continue, // We found the yaml
                        };
                    }
                }
            }
        };
    }
    let elapsed = start.elapsed();
    info!("Post DB initialised, took {:?}", elapsed);
    Ok(())
}
