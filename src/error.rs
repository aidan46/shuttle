use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Path error {0}")]
    Path(String),
    #[error("IPFS error {0}")]
    Ipfs(String),
    #[error("Ethereum error {0}")]
    Ethereum(String),
}
