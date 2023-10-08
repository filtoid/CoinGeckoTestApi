use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

pub async fn establish_connection() -> Result<PgPool, Box<dyn std::error::Error>> {
    dotenv().ok();
    let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Ok(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&conn_str).await?
    )
}

#[cfg(test)]
mod db_connection_tests {
    use super::establish_connection;
    use sqlx::PgPool;

    #[tokio::test]
    async fn connection_success() {
        let _ = establish_connection().await;
    }

}

