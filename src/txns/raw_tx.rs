use std::collections::HashMap;

use bitcoin::{Address, Amount, Network, address::NetworkUnchecked};
use bitcoincore_rpc::{RpcApi, jsonrpc::serde_json};

mod rpc;

fn main() {
    let rpc = rpc::rpc_client();
    let address = "bcrt1qkuhuv5ydxd2e5yvpdt7qq28lthxxlaplgaq3rq"
        .parse::<Address<NetworkUnchecked>>()
        .unwrap()
        .require_network(Network::Regtest)
        .unwrap();
    let amount = Amount::from_btc(1.1).unwrap();
    let fee = Amount::from_btc(0.0001).unwrap();
    let fee = Amount::from_sat(1_000);
    let utxos = rpc.list_unspent(None, None, None, None, None).unwrap();

    let utxo = &utxos[0];

    let inputs = vec![bitcoincore_rpc::json::CreateRawTransactionInput {
        txid: utxo.txid,
        vout: utxo.vout,
        sequence: None,
    }];

    let mut outputs: HashMap<String, Amount> = HashMap::new();
    outputs.insert(address.to_string(), utxo.amount - fee);

    let raw = rpc
        .create_raw_transaction_hex(&inputs, &outputs, None, None)
        .unwrap();

    println!("Raw : {}",raw);

    let funded = rpc.fund_raw_transaction(&*raw, None, None).unwrap();
    let signed = rpc.sign_raw_transaction_with_wallet(&funded.hex, None, None).unwrap();

    let txid = rpc.send_raw_transaction(&signed.hex).unwrap();
    println!("Raw TX sent: {}", txid);
}
