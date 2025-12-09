use std::time::Duration;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub type ConnectionPool = Pool<Postgres>;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn new_pool(
        connection_string: &str,
        run_migrations: bool,
        min_conn: u32,
        max_conn: u32,
    ) -> anyhow::Result<ConnectionPool> {
        let pool = PgPoolOptions::new()
            .min_connections(min_conn)
            .max_connections(max_conn)
            .acquire_timeout(Duration::from_secs(2))
            .idle_timeout(Duration::from_secs(300)) 
            .max_lifetime(Duration::from_secs(1800)) 
            .test_before_acquire(false)
            .connect(connection_string)
            .await
            .map_err(|err| anyhow::anyhow!("Failed to create database connection pool: {}", err))?;

        if run_migrations {
            sqlx::migrate!("../../migrations")
                .run(&pool)
                .await
                .map_err(|err| anyhow::anyhow!("Failed to run database migrations: {}", err))?;
        }

        Ok(pool)
    }
}
