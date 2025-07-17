// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        image -> Nullable<Text>,
        content_path -> Text,
        blog_finished -> Bool,
        project_finished -> Bool,
        hiatus_since -> Nullable<Timestamp>,
        modified -> Timestamp,
        collection -> Nullable<Text>,
    }
}

diesel::table! {
    blog_tags (id) {
        id -> Integer,
        tag -> Text,
        project -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blog_posts,
    blog_tags,
);
