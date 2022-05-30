
use anyhow::{Context, Result, Error};
use clap::Parser;
mod fetch;
use fetch::fetch_loop;

#[derive(Parser, Debug)]
#[clap(author, version, about = "一款拉取 solana 交易记录的工具", long_about = None)]
struct Args {
    #[clap(short = 'a', long = "address", default_value = "DwGdrSQ8ubPo2FJsWArVdjavsMDrwPzpNye2sPT7oqF3")]
    address: String,

    #[clap(short = 't', long = "token-address", default_value = "AFbX8oGjGpmVFywbVouvhQSRmiW2aR1mohfahi4Y2AdB")]
    token_address: String,

    #[clap(short = 'o', long = "output", default_value = "./output.csv")]
    output: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let args: Args = Args::parse();

    block_until_sigint::block(fetch_loop(args.address.clone(), args.token_address.clone(), args.output.clone())).await.context("block_until_sigint error")?;

    log::info!("Finish shutdown.");

    Ok(())
}


