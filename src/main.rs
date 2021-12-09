pub use blc_rs::builder::Builder;
pub use eth2::types::{ChainSpec, MainnetEthSpec};

#[tokio::main]
async fn main() {
    println!("Initializing beacon light client...");
    
    let node_url = "http://localhost:8001";
    let builder = Builder::new(node_url);
    let chain_spec = ChainSpec::mainnet();

    builder.run::<MainnetEthSpec>(&chain_spec).await;
}