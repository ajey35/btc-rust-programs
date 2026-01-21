

use bitcoincore_rpc::RpcApi;
use bitcoin::{Address, Amount, Network, address::{NetworkUnchecked}};

mod rpc;

fn main() {
    let rpc = rpc::rpc_client();
    let address = "bcrt1qkuhuv5ydxd2e5yvpdt7qq28lthxxlaplgaq3rq".parse::<Address<NetworkUnchecked>>().unwrap().require_network(Network::Regtest).unwrap();
    let amount = Amount::from_btc(1.1).unwrap();

// Normal Txn 

    let txid = rpc.send_to_address(
    &address,
    amount,
    Some("Happy-Birthday!"),
    None,
    Some(false), // subtract_fee_from_amount
    None,
    Some(1),     // conf_target
    None,
).unwrap();

    println!("TX_ID : {:?}",txid);

    let mempool = rpc.get_raw_mempool().unwrap();
    println!("Size of Mempool : {:?}",mempool.len());

    let tx = rpc.get_transaction(&txid,None).unwrap();
    println!("Confirmations : {}",tx.info.confirmations);

}
