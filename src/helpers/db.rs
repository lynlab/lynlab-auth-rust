use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel;

pub type Pool = r2d2::Pool<r2d2_diesel::ConnectionManager<MysqlConnection>>;

/// Initialize the database pool.
pub fn init_pool(database_url: &str) -> Result<Pool, r2d2::Error> {
    let manager = r2d2_diesel::ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder().max_size(15).build(manager)
}
