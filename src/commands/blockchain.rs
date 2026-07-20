use crate::error::Result;
use crate::rpc::RpcClient;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockchainInfoResponse{
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

pub async fn run(client: &RpcClient) -> Result <()> {
    let info: BlockchainInfoResponse = client.call("getblockchaininfo", vec![]).await?;
    println!("{}", serde_json::to_string_pretty(&info)?);
    Ok(())  
}