use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::MysqlConnection;
use dotenvy::dotenv;
use std::env;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn establish_connection() -> MysqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).unwrap_or_else(|_| panic!("Could not create database pool"))
}

pub struct Database {
    pub pool: MysqlPool,
}

impl Database {
    pub fn new() -> Database {
        Database {
            pool: establish_connection(),
        }
    }
}
