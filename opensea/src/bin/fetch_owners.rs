extern crate opensea;

use anyhow::{Context, Result, Error};
use serde::{Deserialize, Serialize};
use opensea::util::fetch_owners;

#[derive(Deserialize, Serialize, Debug)]
struct NeedDataType {
    pub address: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut wtr = csv::Writer::from_path("addr.csv").unwrap();
    
    let owners = fetch_owners("https://mainnet.infura.io/v3/b755a6796f2c4c97ad9cfec0a88e37ba".to_string()).await?;
    
    for owner in owners {
        wtr.serialize(NeedDataType {
            address: owner,
        }).unwrap();
    }
    wtr.flush();

    Ok(())
}
