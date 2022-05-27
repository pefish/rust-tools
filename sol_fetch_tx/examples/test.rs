
extern crate sol_fetch_tx;
use anyhow::{Result, Ok};

use sol_fetch_tx::module::hello;

#[tokio::main]
async fn main() -> Result<()> {

    println!("{}", hello().await);
    
    
    Ok(())
}
