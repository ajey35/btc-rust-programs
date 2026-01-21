use bitcoincore_rpc::RpcApi;

mod rpc;

fn chain_info() {
    let rpc = rpc::rpc_client();
    let info = rpc.get_blockchain_info().unwrap();
    println!("Chain ID : {}",info.chain);
    println!("Fulll Info : {:#?}",info);
}
