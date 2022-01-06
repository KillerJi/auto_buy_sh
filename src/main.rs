use crate::error::FomoError;
use chrono::prelude::*;
use secp256k1::SecretKey;
use std::str::FromStr;
use web3::{
    contract::{Contract, Options},
    types::{Address, H160, U256},
};
use log::info;
use log4rs;

extern crate chrono;
mod error;
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let _ = env_logger::try_init();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("running");
    let http = web3::transports::Http::new("http://18.180.227.173:8545/")?;
    let web3 = web3::Web3::new(http);

    let prvk =
        SecretKey::from_str("13962cc606545b8a706ee4fad4ccf6cfd21add41e24f4c9abd667ceeaa0a74aa")
            .unwrap();
    let sender = Address::from_str("0xd5e7C82aAC9ECEee6A47f3AA04dBC613cC427A1E").unwrap();
    let contract = "0xc4dFb1bc3542079BA3F3Cd6168f3eE371ffF66B0";
    let contract = H160::from_str(contract).map_err(|_| FomoError::ConversionFailed)?;
    let contract = Contract::from_json(web3.eth(), contract, include_bytes!("../res/fomo.json"))?;
    let dt = Local::now();
    let current_time = U256::from(dt.timestamp());

    let rounds: U256 = contract
        .query("rounds", (), None, Options::default(), None)
        .await?;
    let time_stamp: U256 = contract
        .query("end_time", (rounds,), None, Options::default(), None)
        .await?;

    if current_time >= time_stamp {
        info!("current_time >= end_time  bnbBuying");
        let key_final_price: U256 = contract
            .query("key_final_price", (), None, Options::default(), None)
            .await?;
        let mut options = Options::default();
        // options.value = Some(key_final_price);
        options.value = Some(U256::from(1));
        let _tx = contract
            .signed_call_with_confirmations(
                "bnbBuy",
                (U256::from(1), U256::from(1), U256::from(1), sender),
                options.clone(),
                1,
                &prvk,
            )
            .await;
    }
    // log::info!("mint receipt: {:?}", tx);
    Ok(())
}
