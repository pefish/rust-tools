use std::time::Duration;

use anyhow::{Context, Result, Error};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
pub struct HttpResult<T> {
    pub message: String,
    pub result: T,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct Tx {
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NeedDataType {
    pub address: String,
    pub from_count: i32,
    pub to_count: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    
    // https://api.etherscan.io/api?module=account&action=tokennfttx&address=0x494c4dd54287b6f86208e9aba1bfa55616423d09
    
    let mut reader = csv::Reader::from_path("addr.csv").unwrap();

    let mut wtr = csv::Writer::from_path("addr_count.csv").unwrap();
    for result in reader.records().into_iter() {
        let record = result.unwrap();
        let address = record[0].to_string();
        println!("{}", address);

        let client = Client::new();
        let mut result = client.get(
            Url::parse(&format!("https://api.etherscan.io/api?module=account&action=tokennfttx&address={}&apikey=WDF9SBXFCPJKSBD9QEA59B2FDJIFMYTDGJ", address)).unwrap()
        )
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36")
        .send()
        .await;
        if result.is_err() {
            result = client.get(
                Url::parse(&format!("https://api.etherscan.io/api?module=account&action=tokennfttx&address={}&apikey=WDF9SBXFCPJKSBD9QEA59B2FDJIFMYTDGJ", address)).unwrap()
            )
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36")
            .send()
            .await;
            if result.is_err() {
                result = client.get(
                    Url::parse(&format!("https://api.etherscan.io/api?module=account&action=tokennfttx&address={}&apikey=WDF9SBXFCPJKSBD9QEA59B2FDJIFMYTDGJ", address)).unwrap()
                )
                .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36")
                .send()
                .await;
                if result.is_err() {
                    panic!("{:?}", result.err());
                }
            }

        }
        let res = result.unwrap();
        let data = res.json::<HttpResult<Vec<Tx>>>().await.unwrap();
        if data.status != "1" {
            panic!("{:?}", "data.status false");
        }
        let mut from_count = 0;
        let mut to_count = 0;
        for item in data.result {
            if item.from == address {
                from_count += 1;
            }
            if item.to == address {
                to_count += 1;
            }
        }
        wtr.serialize(NeedDataType {
            address,
            from_count,
            to_count,
        }).unwrap();
        wtr.flush();

        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
