use sqlx::postgres::PgPool;
use std::{thread, time::Duration};
use std::env;
use log::debug;
mod api_loader;

pub async fn start_coin_updater_thread(pool: PgPool) {
    let delay_string = env::var("CG_API_DELAY").expect("CG_API_DELAY must be set");
    let delay: u64 = delay_string.parse().unwrap();
    debug!("Delay between requests for CG Api {}", delay);
    loop {
        api_loader::refresh_coin_data(pool.clone()).await;

        thread::sleep(Duration::from_millis(delay));
    }
}