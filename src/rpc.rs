use bitcoincore_rpc::{Auth, Client};

pub fn rpc_client()->Client{
    Client::new(
        "http://127.0.0.1:18443",
        Auth::CookieFile(("/home/ajey/.bitcoin/regtest/.cookie").into()),        
    ).expect("RPC Connection Failed")
}