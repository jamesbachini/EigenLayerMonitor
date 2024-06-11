use ethers::prelude::*;
use ethers::utils::hex;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::runtime::Runtime;

const INFURA_API_KEY: &str = "";
const POOL_ADDRESS: &str = "0x858646372CC42E1A627fcE94aa7A7033e7CF075A";
const TOPIC: &str = "0x7cfff908a4b583f36430b25d75964c458d8ede8a99bd61be750e97ee1b2f3a96";

async fn get_block_24_hours_ago(provider: Arc<Provider<Http>>) -> Result<U64, Box<dyn std::error::Error>> {
    let current_block_number = provider.get_block_number().await?;
    let average_block_time = 13; // average block time in seconds
    let blocks_24_hours_ago = (24 * 60 * 60) / average_block_time;
    let estimated_block_number = current_block_number - blocks_24_hours_ago;
    println!("Target Start Block: {}", estimated_block_number);
    Ok(estimated_block_number)
}

async fn calculate_deposits(provider: Arc<Provider<Http>>, pool_address: &str, topic: &str, from_block: U64) -> Result<usize, Box<dyn std::error::Error>> {
    let filter = Filter::new()
        .address(pool_address.parse::<Address>()?)
        .from_block(from_block)
        .to_block(BlockNumber::Latest)
        .topic0(ValueOrArray::Value(H256::from_slice(&hex::decode(topic.trim_start_matches("0x"))?)));
    
    let logs = provider.get_logs(&filter).await?;
    Ok(logs.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    runtime.block_on(async {
        let provider = Arc::new(Provider::<Http>::try_from(format!("https://mainnet.infura.io/v3/{}", INFURA_API_KEY))?);
        let from_block = get_block_24_hours_ago(provider.clone()).await?;
        let total_depositors = calculate_deposits(provider.clone(), POOL_ADDRESS, TOPIC, from_block).await?;
        println!("Total Depositors: {}", total_depositors);
        Ok(())
    })
}
