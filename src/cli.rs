use crate::{error::Error, eth::EthRegister, ipfs::ipfs_upload};

use clap::Parser;
use std::{fs::File, net::Ipv4Addr, path::PathBuf};
use tracing::info;
use web3::types::Address;

#[derive(Debug, Parser)]
pub struct Cli {
    path: PathBuf,
    /// IPFS HTTP port
    #[arg(long, default_value = "5001")]
    ipfs_port: u16,
    /// IPFS HTTP ip
    #[arg(long, default_value = "127.0.0.1")]
    ipfs_host: Ipv4Addr,
    #[arg(long, default_value = "8545")]
    eth_port: u16,
    #[arg(long, default_value = "localhost")]
    eth_host: String,
    #[arg(long, default_value = "0x4d470146215d085c75767b717dbB8D3b4468F893")]
    addr: Address,
    #[arg(short, long, default_value = "false")]
    /// Print debug logs
    pub debug: bool,
}

impl Cli {
    pub async fn run(&self) -> Result<(), Error> {
        let path = &self.path;
        let host = self.ipfs_host;
        let port = self.ipfs_port;
        info!("Trying to open file {} ...", path.display());
        let cid = match File::open(path) {
            Ok(file) => ipfs_upload(file, port, host).await?,
            Err(e) => return Err(Error::Path(e.to_string())),
        };
        println!("Upload {} to IPFS complete", path.display());
        println!("CID = {cid}");
        let url = format!("http://{}:{}", self.eth_host, self.eth_port);
        let eth = EthRegister::new(&url, self.addr).await?;
        let tx = eth.set_cid(&cid).await?;
        println!("Transaction {tx:?} complete");
        println!("Upload to contract {:?} complete", self.addr);
        Ok(())
    }
}
