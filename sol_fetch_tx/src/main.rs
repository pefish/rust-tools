
use anyhow::{Context, Result, Error};
use clap::Parser;
mod fetch;
use fetch::fetch_loop;

#[derive(Parser, Debug)]
#[clap(author, version, about = "一款拉取 solana 交易记录的工具", long_about = None)]
struct Args {
    #[clap(short = 't', long = "token-account", default_value = "H9mhqbPkymytwsxorawVYpsLybJB56NZFX3ChYEyWEev")]
    token_account: String,
    #[clap(short = 'o', long = "output", default_value = "./output.csv")]
    output: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let args: Args = Args::parse();

    block_until_sigint::block(fetch_loop(args.token_account.clone(), args.output.clone())).await.context("block_until_sigint error")?;

    log::info!("Finish shutdown.");

    Ok(())
}


