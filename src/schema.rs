table! {
    actions (memeid, userid) {
        memeid -> Int4,
        userid -> Int4,
        is_upvote -> Bool,
        is_active -> Bool,
        posted_at -> Timestamp,
    }
}

table! {
    memes (memeid) {
        memeid -> Int4,
        author -> Int4,
        image -> Varchar,
        upvote -> Int4,
        downvote -> Int4,
        score -> Float4,
        heat -> Float4,
        last_action -> Timestamp,
        posted_at -> Timestamp,
    }
}

table! {
    meme_tags (tagid, memeid) {
        tagid -> Int4,
        memeid -> Int4,
    }
}

table! {
    tags (tagid) {
        tagid -> Int4,
        tagname -> Varchar,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        username -> Varchar,
        userupvote -> Int4,
        userdownvote -> Int4,
        userscore -> Float4,
    }
}

joinable!(actions -> memes (memeid));
joinable!(actions -> users (userid));
joinable!(meme_tags -> memes (memeid));
joinable!(meme_tags -> tags (tagid));
joinable!(memes -> users (author));

allow_tables_to_appear_in_same_query!(
    actions,
    memes,
    meme_tags,
    tags,
    users,
);
