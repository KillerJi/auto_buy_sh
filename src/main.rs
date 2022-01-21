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
use std::time::Duration;

mod error;
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("/home/lx/fomo_auto_buy/auto_buy_sh/log4rs.yaml", Default::default()).unwrap();
    info!("running");
    let http = web3::transports::Http::new("http://18.180.227.173:8545/")?;
    let web3 = web3::Web3::new(http);
    let prvk =
        SecretKey::from_str("b2defe7b9a914556fca1fd2b7dd2846b713fea42ea8e42caefa1cf8e507ce7d3")
            .unwrap();
    let sender = Address::from_str("0xD416Ec3B90B5CaEf891DF3caDc9887539AE0928c").unwrap();
    let contract = "0xD696d4C95114Ee2EBaD105787A8Ce55d6AF3Dc2F";
    let contract = H160::from_str(contract).map_err(|_| FomoError::ConversionFailed)?;
    let contract = Contract::from_json(web3.eth(), contract, include_bytes!("../res/fomo.json"))?;
    loop {
        // nohup /home/lx/fomo_auto_buy/auto_buy_sh/target/release/auto_buy &
        let dt = Local::now();
        let current_time = U256::from(dt.timestamp());
        let rounds: U256 = contract
            .query("rounds", (), None, Options::default(), None)
            .await?;
        let end_time_stamp: U256 = contract
            .query("end_time", (rounds,), None, Options::default(), None)
            .await?;
        if current_time >= end_time_stamp {
            info!("rounds {:?}",rounds);
            info!("end_time_stamp {:?}",end_time_stamp);
            info!("current_time >= end_time  bnbBuying");
            // let key_final_price: U256 = contract
            //     .query("key_final_price", (), None, Options::default(), None)
            //     .await?;
            // println!("key_final_price {:?}",key_final_price);
            let mut options = Options::default();
            options.gas = Some(U256::from(1_000_000));
            options.value = Some(U256::from(40000000000000000u64));
            println!("options {:?} ",options);
            let _tx = contract
                .signed_call_with_confirmations(
                    "bnbBuy",
                    (U256::from(1), U256::from(1), sender),
                    options.clone(),
                    1,
                    &prvk,
                )
                .await;
                log::info!(" receipt: {:?}", _tx);
        }
        std::thread::sleep(Duration::from_millis(30000));
        //  log::info!("mint receipt: {:?}", tx);
        // let _ = env_logger::try_init();
    }
    Ok(())
}
