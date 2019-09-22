table! {
    memes (memeid) {
        memeid -> Int4,
        author -> Int4,
        image -> Varchar,
        image_data -> Bytea,
        upvote -> Int4,
        downvte -> Int4,
        date -> Timestamp,
        heat -> Nullable<Float8>,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        username -> Varchar,
        userupvote -> Nullable<Int4>,
        userdownvote -> Nullable<Int4>,
    }
}

joinable!(memes -> users (author));

allow_tables_to_appear_in_same_query!(
    memes,
    users,
);
