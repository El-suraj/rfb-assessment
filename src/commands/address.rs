use crate::error::Result;
use crate::rpc::RpcClient;

pub async fn run_new_address(client: &RpcClient) -> Result <()> {
    let address: String =  client.call("getnewaddress", vec![]).await?;
    println!("New Address: {}", address);
    Ok(())
}