table! {
    memes (memeid) {
        memeid -> Int4,
        author -> Int4,
        image -> Varchar,
        upvote -> Int4,
        downvote -> Int4,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        username -> Varchar,
        userupvote -> Int4,
        userdownvote -> Int4,
        testbool -> Bool,
    }
}

joinable!(memes -> users (author));

allow_tables_to_appear_in_same_query!(
    memes,
    users,
);
