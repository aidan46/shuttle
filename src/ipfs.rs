use crate::error::Error;

use ipfs_api::{IpfsApi, IpfsClient, TryFromUri};
use std::{
    fs::File,
    net::{IpAddr, Ipv4Addr},
};
use tracing::info;

/// Uploads a file to IPFS and returns the CID
pub async fn ipfs_upload(file: File, port: u16, host: Ipv4Addr) -> Result<String, Error> {
    let addr = format!("/ip4/{}/tcp/{}", IpAddr::V4(host), port);
    info!("Connecting to {addr} ...");
    let client = match IpfsClient::from_multiaddr_str(&addr) {
        Ok(client) => client,
        Err(e) => return Err(Error::Ipfs(e.to_string())),
    };
    match client.add(file).await {
        Ok(file) => Ok(file.hash),
        Err(e) => Err(Error::Ipfs(e.to_string())),
    }
}
