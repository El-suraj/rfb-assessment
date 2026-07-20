use crate::config::Config;
use crate::error::{AppError, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    id: &'static str,
    method: String,
    params: Vec<Value>,
}

#[derive(Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcErrorDetail>,
}

#[derive(Deserialize, Debug)]
pub struct RpcErrorDetail {
    pub code: i64,
    pub message: String,
}

#[derive(Clone)]
pub struct RpcClient {
    client: Client,
    config: Config,
}

impl RpcClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn call<T: DeserializeOwned>(&self, method: &str, params: Vec<Value>) -> Result<T> {
        let req_payload = RpcRequest {
            jsonrpc: "1.0",
            id: "rfb-client",
            method: method.to_string(),
            params,
        };

        let response = self
            .client
            .post(self.config.url())
            .basic_auth(&self.config.rpc_user, Some(&self.config.rpc_pass))
            .header("Content-Type", "application/json")
            .json(&req_payload)
            .send()
            .await
            .map_err(|e| AppError::Command(format!("Connection failure: Could not connect to Bitcoin Core at {}. Error: {}", self.config.url(), e)))?;

        let rpc_res: RpcResponse<T> = response.json().await?;

        if let Some(err) = rpc_res.error {
            return Err(AppError::Rpc {
                code: err.code,
                message: err.message,
            });
        }

        rpc_res
            .result
            .ok_or_else(|| AppError::Command("RPC returned empty result".to_string()))
    }
}