use bitcoincore_rpc::RpcApi;

mod rpc;

fn create_and_mint_wallet() {
    let rpc = rpc::rpc_client();
    let _ = rpc.create_wallet("ajay", None, None, None, None);
    // rpc.load_wallet("ajay").unwrap();
    let address = rpc.get_new_address(None, None).unwrap().assume_checked();
    println!("Your Address : {:?}", address);
    println!("Mining the Blocks......");
    rpc.generate_to_address(1, &address).unwrap();

    let balance = rpc.get_balance(None, None).unwrap();
    println!("Balance : {}", balance);
}
