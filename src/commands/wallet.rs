use crate::error::Result;
use crate::rpc::RpcClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfoResponse {
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    pub balance: f64,
    #[serde(rename = "unconfirmed_balance")]
    pub unconfirmed_balance: f64,
    pub txcount: u64,
}

pub async fn run_info(client: &RpcClient) -> Result<()> {
    let info: WalletInfoResponse = client.call("getwalletinfo", vec![]).await?;
    println!("{}", serde_json::to_string_pretty(&info)?);
    Ok(())
}

pub async fn run_balance(client: &RpcClient) -> Result<()> {
    let balance: f64 = client.call("getbalance", vec![]).await?;
    println!("Wallet Balance: {} BTC", balance);
    Ok(())
}