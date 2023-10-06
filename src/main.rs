extern crate log;
extern crate pretty_env_logger;

mod server;
mod schema;

#[tokio::main]
async fn main () {
    pretty_env_logger::init();

    // Start api server to handle requests
    server::start(([127,0,0,1], 3030)).await;
}