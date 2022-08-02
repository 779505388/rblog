table! {
    article (id) {
        id -> Integer,
        title -> Varchar,
        brief -> Varchar,
        user_id -> Nullable<Integer>,
        url_en -> Varchar,
        text -> Text,
        template -> Text,
        image_url -> Varchar,
        like_count -> Nullable<Integer>,
        views -> Nullable<Integer>,
        created -> Datetime,
        modified -> Datetime,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        nickname -> Varchar,
        mail -> Varchar,
        mail_hash -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    article,
    user,
);
