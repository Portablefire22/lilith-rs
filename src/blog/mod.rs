use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::blog_posts;

pub mod route;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Post {
    /// Name of the blog post
    pub(crate) name: String,
    /// Short description shown on preview
    pub(crate) description: String,
    /// Blog Post Icon
    pub(crate) image: Option<String>,
    /// Tags for filtering posts
    pub(crate) tags: Vec<String>,
    /// Path of the post's content.md
    pub(crate) content_path: String,
    /// Is the blog writing finished?
    #[serde(default)]
    pub(crate) blog_finished: bool,
    /// Is the project this blog associated with finished?
    #[serde(default)]
    pub(crate) project_finished: bool,
    /// When was the project put on pause?
    #[serde(with = "date_option")]
    #[serde(default)]
    pub(crate) hiatus_since: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=blog_posts)]
pub(crate) struct InsertPost {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image: Option<String>,
    pub(crate) content_path: String,
    pub(crate) blog_finished: bool,
    pub(crate) project_finished: bool,
    pub(crate) hiatus_since: Option<NaiveDateTime>,
    pub(crate) modified: NaiveDateTime,
}

#[derive(Queryable, Selectable, Serialize, PartialEq, Eq, PartialOrd)]
#[diesel(table_name=blog_posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct QueryPost {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image: Option<String>,
    pub(crate) content_path: String,
    pub(crate) blog_finished: bool,
    pub(crate) project_finished: bool,
    #[serde(with = "date_option")]
    pub(crate) hiatus_since: Option<NaiveDateTime>,
    #[serde(with = "date")]
    pub(crate) modified: NaiveDateTime,
}


mod date {
    use std::str::FromStr;

    use chrono::{NaiveDateTime};
    use log::info;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deseria: String = String::deserialize(deserializer)?;
        info!("{:?}", &deseria);
        match NaiveDateTime::parse_from_str(&deseria, "%Y-%m-%d %H:%M:%S") {
            Ok(ok) => Ok(ok),
            Err(err) => Err(serde::de::Error::custom(format!("could not convert String to NaiveDateTime: {:?}", err))),
        }
    }

    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.and_utc().to_string())
    }
}

mod date_option {
    use std::str::FromStr;

    use chrono::{NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deseria: String = match Option::deserialize(deserializer)? {
            None => return Ok(None),
            Some(de) => de,
        };
        info!("{:?}", &deseria);
        match NaiveDateTime::parse_from_str(&deseria, "%Y-%m-%d %H:%M:%S") {
            Ok(ok) => Ok(Some(ok)),
            Err(err) => Err(serde::de::Error::custom(format!("could not convert String to NaiveDateTime: {:?}", err))),
        }
    }

    pub fn serialize<S>(
        date: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(unwrapped) => {
                serializer.serialize_str(&unwrapped.and_utc().to_string())
            },
            None => serializer.serialize_none()
        }
    }
}
