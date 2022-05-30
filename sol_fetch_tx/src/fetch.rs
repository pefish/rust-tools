use std::time::Duration;
use std::{str::FromStr};
use rust_decimal::Decimal;
use rust_decimal::prelude::{ToPrimitive, FromPrimitive};
use tokio::time;
use reqwest::{Client, ClientBuilder, Url};
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result, Error};

#[derive(Deserialize, Debug)]
pub struct HttpResult<T> {
    pub succcess: bool,
    pub data: T,
}

#[derive(Deserialize, Debug)]
struct DataType {
    pub begin: u64,
    pub tx: TxType,
}

#[derive(Deserialize, Debug)]
struct TxType {
    pub hasNext: bool,
    pub total: u64,
    pub transactions: Vec<TxItemType>,
}

#[derive(Deserialize, Debug)]
struct TxItemType {
    pub blockTime: u64,
    pub status: String,  // Success
    pub txHash: String,
    pub signer: Vec<String>,
    pub change: ChangeType,
}

#[derive(Deserialize, Debug)]
struct ChangeType {
    pub address: String,
    pub changeAmount: Decimal,  // 可能是个字符串
    pub changeType: String, // inc/dec
    pub decimals: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct NeedDataType {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: Decimal,
}

pub async fn fetch_loop (token_account: String, output: String) -> () {
    let mut inte = time::interval(Duration::from_secs(5));
    let mut offset = 0;
    let limit = 10;

    let mut wtr = csv::Writer::from_path(output).unwrap();

    loop {
        inte.tick().await;
        log::info!("offset: {}", offset);

        let client = Client::new();
        let result = client.get(
            Url::parse(&format!("https://api.solscan.io/account/token/txs?address={}&offset={}&limit={}", token_account, offset, limit)).unwrap()
        )
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36")
        .send()
        .await;
        if result.is_err() {
            panic!("{:?}", result.err());
        }
        let res = result.unwrap();
        let data = res.json::<HttpResult<DataType>>().await.unwrap();
        if !data.succcess {
            panic!("{:?}", "data.succcess false");
        }
        let txs = data.data.tx.transactions;
        for tx in txs {
            if tx.status == "Success".to_string() {
                let mut to = tx.change.address.clone();
                let mut from = tx.signer[0].clone();
                let mut amount = Decimal::new(tx.change.changeAmount.to_i64().unwrap(), tx.change.decimals as u32).normalize();
                // log::info!("{}, {}, {}", tx.change.changeAmount, tx.change.decimals, amount);
                if tx.change.changeType == "dec" {
                    let tmp = from;
                    from = to;
                    to = tmp;
                    amount = -amount;
                }
                if amount < Decimal::from(0) {
                    let tmp = from;
                    from = to;
                    to = tmp;
                    amount = -amount;
                }
                wtr.serialize(NeedDataType {
                    tx_hash: tx.txHash.clone(),
                    from,
                    to,
                    amount,
                }).unwrap();
            }
        }
        if !data.data.tx.hasNext {
            log::info!("{}", "done!!!");
            break;
        }

        offset += limit;
        // break;
    }

    wtr.flush();
}
