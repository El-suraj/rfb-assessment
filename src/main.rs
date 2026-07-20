mod cli;
mod commands;
mod config;
mod error;
mod rpc;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use error::Result;
use rpc::RpcClient;
use serde_json::{json, Value};

#[tokio::main]
async fn main (){
    if let Err(err) = run_app().await{
        eprintln!("\x1b[1;3imError:\x1b[0m {}", err);
        std::process::exit(1);
    }
}

async fn run_app() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from_env()?;
    let client = RpcClient::new(config);

    match cli.command {
        Commands::BlockchainInfo => {
           commands::blockchain::run(&client).await?;
        }
        Commands::WalletInfo => {
            commands::wallet::run_info(&client).await?;
        }
        Commands::Balance => {
            commands::wallet::run_balance(&client).await?;
        }
        Commands::NewAddress => {
            commands::address::run_new_address(&client).await?;
        }
        Commands::Rpc {method, params} => {
            let json_params: Vec<Value> = params.into_iter()
            .map(|p|{
                if let Ok(num) = p.parse::<i64>(){
                    json!(num)
                }else if let Ok(boolean) = p.parse::<bool>(){
                    json!(boolean)
                }else {
                    json!(p)
                }
            })
            .collect();

        let result: Value = client.call(&method, json_params).await?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }
    Ok(())
}
