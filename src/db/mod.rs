use dotenv::dotenv;
use std::env;

#[macro_export]
macro_rules! conn {
    () => {
        db::POOL.get().unwrap()
    };
}

embed_migrations!();

pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
lazy_static::lazy_static!{
    /// Connection pool to PostgreSQL DB
    pub static ref POOL: Pool = establish_connection_pool();
}

///Read database url from .env and return a connection pool to it
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env !");
    
    let conn_man = diesel::r2d2::ConnectionManager::new(db_url);
    
    let pool = diesel::r2d2::Pool::new(conn_man)
    .unwrap_or_else(|_| panic!("Error connecting to database"));
    
    embedded_migrations::run(&pool.clone().get().unwrap()).unwrap();
    pool
}

// ///Read database url from .env and connect to it
// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env !");

//     let conn = PgConnection::establish(&db_url)
//         .unwrap_or_else(|_| panic!("Error connecting to database {}", db_url));

//     embedded_migrations::run(&conn).unwrap();
//     conn
// }
