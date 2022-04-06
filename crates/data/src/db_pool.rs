use dotenv::dotenv;
use std::env;

use anyhow::{Context, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use tracing::{debug, info};

embed_migrations!("./migrations");

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> Result<ConnectionPool> {
    // load .env file
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").context("Failed to obtain DATABASE_URL variable")?;

    debug!("Connecting to DB: {:?}", db_url);

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(manager)
        .context("Error building a connection pool")?;

    let connection = pool.get().expect("Failed to obtain database connection");

    info!("Running migrations");
    embedded_migrations::run(&connection).context("Failed to run migrations")?;
    info!("Migrations executed successfully");

    Ok(pool)
}
