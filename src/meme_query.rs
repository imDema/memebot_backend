use crate::schema::{memes, meme_tags};
use crate::models::Meme;
use crate::db::POOL;
use diesel::prelude::*;
use chrono::prelude::*;

enum Order {
    None,
    Date,
    Heat,
    Rating,
}

// enum Range {
//     All,
//     In(usize,usize),
// }

enum Filter {
    None,
    User(i32),
    Tag(i32),
}

pub struct MemeQuery {
    order: Order,
    // range: Range,
    filter: Filter,
}

impl MemeQuery {
    pub fn new() -> Self {
        MemeQuery{
            order: Order::None,
            // range: Range::All,
            filter: Filter::None,
        }
    }

    pub fn by_tag(mut self, tagid: i32) -> Self {
        self.filter = Filter::Tag(tagid);
        self
    }

    pub fn by_user(mut self, userid: i32) -> Self {
        self.filter = Filter::User(userid);
        self
    }

    // fn range(mut self, from: usize, to: usize) -> Self { //TODO make public
    //     self.range = Range::In(from, to);
    //     self
    // }

    pub fn order_hot(mut self) -> Self {
        self.order = Order::Heat;
        self
    }
    
    pub fn order_date(mut self) -> Self {
        self.order = Order::Date;
        self
    }
    
    pub fn order_rating(mut self) -> Self {
        self.order = Order::Rating;
        self
    }

    pub fn execute(&self) -> QueryResult<Vec<Meme>> {
        let conn = POOL.get().unwrap();
        let base = memes::table;
        let mut filtered = match self.filter {
            Filter::None => base.load::<Meme>(&conn),
            Filter::Tag(id) => base.inner_join(meme_tags::table)
                                    .filter(meme_tags::tagid.eq(id))
                                    .select((memes::memeid,
                                        memes::author,
                                        memes::image,
                                        memes::upvote,
                                        memes::downvote,
                                        memes::score,
                                        memes::heat,
                                        memes::last_action,
                                        memes::posted_at,))
                                    .load::<Meme>(&conn),
            Filter::User(id) => base.filter(memes::author.eq(id)).load::<Meme>(&conn),
        }?;

        let now = Local::now().naive_local();
        match self.order {
            Order::None => (),
            Order::Date => filtered.sort_unstable_by(|a, b| b.posted_at.cmp(&a.posted_at)),
            Order::Heat => {
                filtered.iter_mut().for_each(|m| m.update_heat(&now));
                filtered.sort_unstable_by(|a, b| b.heat.partial_cmp(&a.heat).unwrap())
            },
            Order::Rating => filtered.sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap()),
        };

        Ok(filtered)
        //TODO change this
    }
}