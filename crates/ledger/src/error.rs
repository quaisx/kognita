
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("No Genesis node present in DAG")]
    ERR_NODE_NO_GENESIS,
    #[error("Cannot select DAG tip")]
    ERR_NODE_NO_TIP,
}