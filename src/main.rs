extern crate log;
extern crate pretty_env_logger;

mod processors;
mod server;
mod schema;
mod db_connections;

#[tokio::main]
async fn main () {
    pretty_env_logger::init();
    let pool = db_connections::establish_connection().await.unwrap();

    // Start the coin processor
    tokio::spawn(async move { 
        processors::start_coin_updater_thread(pool.clone()).await 
    });

    let pool = db_connections::establish_connection().await.unwrap();
    let res = sqlx::migrate!("db/migrations")
        .run(&pool)
        .await;
    match res {
        Ok(_) => {log::info!("Migration complete");},
        Err(err) => {
            log::error!("{}", err.to_string());
            panic!("Failed during migration");
        }
    }

    // Start api server to handle requests
    server::start(([127,0,0,1], 3030)).await;
}