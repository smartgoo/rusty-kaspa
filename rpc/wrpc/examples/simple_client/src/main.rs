// Example of simple client to connect with Kaspa node using wRPC connection and collect some node and network basic data

use kaspa_rpc_core::{api::rpc::RpcApi, GetBlockDagInfoResponse};
use kaspa_wrpc_client::{
    client::{ConnectOptions, ConnectStrategy},
    prelude::NetworkId,
    prelude::NetworkType,
    KaspaRpcClient, WrpcEncoding,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    check_node_status().await;
}

async fn check_node_status() {
    // Select encoding method to use, depending on node settings
    let encoding = WrpcEncoding::Borsh;

    // If you want to connect to your own node, define your node address and wRPC port using let url = Some("ws://0.0.0.0:17110")
    // Verify your Kaspa node is runnning with --rpclisten-borsh=0.0.0.0:17110 parameter
    // In this example we don't use a specific node but we connect through the resolver, which use a pool of public nodes
    let url = Some("ws://127.0.0.1:17110");
    let resolver = None;

    // Define the network your Kaspa node is connected to
    // You can select NetworkType::Mainnet, NetworkType::Testnet, NetworkType::Devnet, NetworkType::Simnet
    let network_type = NetworkType::Mainnet;
    let selected_network = Some(NetworkId::new(network_type));

    // Advanced options
    let subscription_context = None;

    // Create new wRPC client with parameters defined above
    let client = KaspaRpcClient::new(encoding, url, resolver, selected_network, subscription_context).unwrap();

    // Advanced connection options
    let timeout = 5_000;
    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(timeout)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };

    // Connect to selected Kaspa node
    client.connect(Some(options)).await.unwrap();

    // Retrieve and show Kaspa node information
    let GetBlockDagInfoResponse { pruning_point_hash, .. } = client.get_block_dag_info().await.unwrap();

    // println!("tip hashes: {:?}", tip_hashes);
    // sleep(Duration::from_secs(10)).await;

    // Use the first virtual parent hash instead of pruning point hash
    // This is more likely to be within the retention period

    let mut low_hash = pruning_point_hash;
    loop {
        let vspc = client.get_virtual_chain_from_block_custom(low_hash).await.unwrap();

        low_hash = *vspc.added_chain_block_hashes.last().unwrap();

        println!("low hash {}", low_hash);
    }

    // Disconnect client from Kaspa node
    // client.disconnect().await.unwrap();
}
