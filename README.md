# CoinGeckoTestApi
A project for testing the API on CoinGecko

### Getting Started
There is a script to start a local database in Docker. To run
this script and start the database use `./run_db.sh`. 

You will need to copy `.env.test` to `.env` and fill in the API
key from [CoinGecko](https://www.coingecko.com/) and then you can
run the application `cargo run`.

### Rate Limiting
The free tier of CoinGecko limits the number of requests to 30 times
per minute. This is a request every 2 seconds (2000 milliseconds).
The processor runs in it's own thread and will read in the value
`CG_API_DELAY` from the `.env` file (minimum value must be 2000ms 
to prevent api throttling on the API).

