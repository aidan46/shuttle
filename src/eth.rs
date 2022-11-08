use crate::error::Error;

use tracing::info;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H160, H256},
    Web3,
};

/// Ethereum connection
pub struct EthRegister {
    contract: Contract<Http>,
    accounts: Vec<H160>,
}

impl EthRegister {
    pub async fn new(url: &str, address: Address) -> Result<Self, Error> {
        info!("Connecting to {url} ...");
        let client = match Http::new(url) {
            Ok(http) => Web3::new(http),
            Err(e) => return Err(Error::Ethereum(e.to_string())),
        };
        info!("Getting accounts ...");
        let accounts = match client.eth().accounts().await {
            Ok(accounts) => accounts,
            Err(e) => return Err(Error::Ethereum(e.to_string())),
        };
        info!("Looking for contract at {address:#?} ...");
        let contract = match Contract::from_json(
            client.eth(),
            address,
            include_bytes!("../artifacts/register.abi"),
        ) {
            Ok(contract) => contract,
            Err(e) => return Err(Error::Ethereum(e.to_string())),
        };

        Ok(Self { contract, accounts })
    }

    pub async fn set_cid(&self, cid: &str) -> Result<H256, Error> {
        info!("Setting CID ...");
        let tx = match self
            .contract
            .call(
                "setCID",
                (cid.to_string(),),
                self.accounts[0],
                Options::default(),
            )
            .await
        {
            Ok(tx) => tx,
            Err(e) => return Err(Error::Ethereum(e.to_string())),
        };
        info!("Transaction complete!");
        info!("tx hash: {tx:?}");
        Ok(tx)
    }
}
