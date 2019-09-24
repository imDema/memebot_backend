table! {
    likes (memeid, userid) {
        memeid -> Int4,
        userid -> Int4,
        liked_at -> Timestamp,
    }
}

table! {
    meme_tags (tagid, memeid) {
        tagid -> Int4,
        memeid -> Int4,
    }
}

table! {
    memes (memeid) {
        memeid -> Int4,
        author -> Int4,
        image -> Varchar,
        upvote -> Int4,
        downvote -> Int4,
        posted_at -> Timestamp,
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
    }
}

joinable!(likes -> memes (memeid));
joinable!(likes -> users (userid));
joinable!(meme_tags -> memes (memeid));
joinable!(meme_tags -> tags (tagid));
joinable!(memes -> users (author));

allow_tables_to_appear_in_same_query!(
    likes,
    meme_tags,
    memes,
    tags,
    users,
);
