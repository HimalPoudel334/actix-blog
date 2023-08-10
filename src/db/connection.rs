use crate::config::ApplicationConfiguration;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::r2d2::{PoolError, PooledConnection};
use diesel::sqlite::SqliteConnection;

pub type SqliteConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
pub type PooledSqliteConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(app_config: &ApplicationConfiguration) -> SqliteConnectionPool {
    let database_url = app_config.database_url.to_owned();

    let conn_manager = ConnectionManager::<SqliteConnection>::new(database_url);
    match Pool::builder().build(conn_manager) {
        Ok(pool) => pool,
        Err(err) => {
            println!("Error while creating database connection pool: {}", err);
            std::process::exit(1);
        }
    }
}

//helper fn to get connection from db pool. The connection function above creates db pool.
//This method gets a single connection from pool
pub fn get_db_connection_from_pool(
    pool: &SqliteConnectionPool,
) -> Result<PooledSqliteConnection, PoolError> {
    let result = pool.get().unwrap();
    Ok(result)
}

// why doesn't this function work?

// //function to setup redis for storing session data
// pub fn get_redis_connection(
//     app_config: &ApplicationConfiguration,
// ) -> SessionMiddleware<SessionStore> {
//     let redis_key_from_string: Key = Key::from(app_config.redis_secret_key.as_bytes());
//     SessionMiddleware::new(
//         RedisActorSessionStore::new(app_config.redis_url.to_owned()),
//         redis_key_from_string.clone(),
//     )
// }
