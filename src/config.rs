use crate::error::{AppError, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_host: String,
    pub rpc_port: u16,
    pub rpc_user: String,
    pub rpc_pass: String,
    pub wallet_name: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let rpc_host = env::var("BITCOIN_RPC_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let rpc_port = env::var("BITCOIN_RPC_PORT")
            .unwrap_or_else(|_| "18443".to_string())
            .parse::<u16>()
            .map_err(|_| AppError::Config("Invalid BITCOIN_RPC_PORT value".to_string()))?;
        let rpc_user = env::var("BITCOIN_RPC_USER").unwrap_or_else(|_| "polaruser".to_string());
        let rpc_pass = env::var("BITCOIN_RPC_PASS").unwrap_or_else(|_| "polarpass".to_string());
        let wallet_name = env::var("BITCOIN_RPC_WALLET").ok();

        Ok(Self {
            rpc_host,
            rpc_port,
            rpc_user,
            rpc_pass,
            wallet_name,
        })
    }

    pub fn url(&self) -> String {
        if let Some(ref wallet) = self.wallet_name {
            format!("http://{}:{}/wallet/{}", self.rpc_host, self.rpc_port, wallet)
        } else {
            format!("http://{}:{}/", self.rpc_host, self.rpc_port)
        }
    }
}