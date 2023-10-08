use sqlx::postgres::PgPool;
use std::{thread, time::Duration};

mod api_loader;

pub async fn start_coin_updater_thread(pool: PgPool) {
    loop {

        api_loader::refresh_coin_data(pool.clone()).await;
        
        // TODO: Read the pause value from env
        thread::sleep(Duration::from_millis(5000));
    }
}