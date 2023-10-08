use log::{info,debug,error,warn};
use sqlx::{postgres::PgPool, error};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::DateTime;
use reqwest;

#[derive(sqlx::FromRow, Debug)]
pub struct CoinModel {
    id: i32,
    name: String,
    cg_id: String,
    symbol: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct PriceModel {
    coin_id: i32,
    usd: f64,
    eur: f64,
    gbp: f64,
    eth: f64,
    btc: f64,
    xrp: f64,
    timestamp: DateTime<chrono::Utc>, 
}

pub(super) async fn refresh_coin_data(pool: PgPool) {
    debug!("Running update coin data");

    // Check if we have any null values
    let rows = sqlx::query_as::<_, CoinModel>(
        "select * from coin where ID not in \
                ( select coin_id from price ) limit 1"
    ) 
        .fetch_one(&pool)
        .await;

    let coin = match rows {
        Ok(res) => {
            info!("{}", res.name);
            res
        },
        Err(_) => {
            warn!("Failed to find any unfilled coin");
            match get_least_fresh_coin(pool.clone()).await {
                Ok(coin) => coin,
                Err(err) => {
                    error!("{}", err.to_string());
                    return;
                }
            }
        }
    };

    get_data_for_coin(coin.id, coin.cg_id, pool.clone()).await;

}

#[derive(Debug, Deserialize, Serialize)]
struct PriceResponse {
    id: String,
    name: String,
    symbol: String,
    market_data: MarketData,
}

impl PriceResponse {
    fn get_empty(cg_id: String) -> PriceResponse {
        PriceResponse { 
            id: cg_id.clone(), 
            name: cg_id.clone(), 
            symbol: cg_id, 
            market_data: 
                MarketData { 
                    current_price: PriceData 
                        { 
                            usd: 0.0, 
                            eur: 0.0, 
                            gbp: 0.0, 
                            eth: 0.0, 
                            btc: 0.0, 
                            xrp: 0.0 
                        }
                } 
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct MarketData {
    current_price: PriceData
}

#[derive(Debug, Deserialize, Serialize)]
struct PriceData {
    usd: f32,
    eur: f32,
    gbp: f32,
    eth: f32,
    btc: f32,
    xrp: f32,
}

async fn get_data_for_coin(coin_id: i32, cg_id: String, pool: PgPool) {
    let api_url = env::var("CG_URL").expect("CG_URL must be set");
    let api_token = env::var("CG_TOKEN").expect("CG_TOKEN must be set");
    let url = format!("{}/coins/{}?x_cg_demo_api_key={}", api_url, cg_id, api_token);

    info!("Making request to: {}",url);
    let tmp = match reqwest::get(url.clone()).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error getting url {}: {}", url, err);
            return;
        }
    };
    
    let pricedata = match tmp.json::<PriceResponse>().await {
        Ok(res) => res,
        Err(err) => {
            error!("{}", err);
            PriceResponse::get_empty(cg_id.clone())
        }
    };

    let res = sqlx::query(
            "INSERT INTO price(coin_id,usd,eur,gbp,eth,btc,xrp) VALUES( \
                $1, $2, $3, $4, $5, $6, $7 \
            )    
            "
        ) 
        .bind(coin_id)
        .bind(&pricedata.market_data.current_price.usd)
        .bind(&pricedata.market_data.current_price.eur)
        .bind(&pricedata.market_data.current_price.gbp)
        .bind(&pricedata.market_data.current_price.eth)
        .bind(&pricedata.market_data.current_price.btc)
        .bind(&pricedata.market_data.current_price.xrp)
        .execute(&pool)
        .await;
    match res {
        Ok(_) => info!("Updated for {} ({})", pricedata.name, pricedata.id),
        Err(err) => error!("{}", err)
    }
    
}

async fn get_least_fresh_coin(pool: PgPool) -> Result<CoinModel, error::Error> {
    let res = sqlx::query_as::<_, PriceModel>(
        "SELECT * from price b where id in ( \
            SELECT DISTINCT ON (coin_id) id \
            FROM  price \
            ORDER  BY coin_id,timestamp DESC \
        ) order by timestamp ASC limit 1"
    ) 
        .fetch_one(&pool)
        .await;

    let price = match res {
        Ok(p) => p,
        Err(err) => {
            return Err(err);
        }
    };

    match sqlx::query_as::<_, CoinModel>(
        "select * from coin where id=$1"
    )
        .bind(&price.coin_id)
        .fetch_one(&pool)
        .await {
            Ok(res) => Ok(res),
            Err(err) => Err(err)
        }
}