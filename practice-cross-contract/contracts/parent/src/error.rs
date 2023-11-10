use cw_utils::ParseReplyError;
use thiserror::Error;
use cosmwasm_std::StdError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("{0}")]
    ParseError(#[from] ParseReplyError),

    #[error("Unrecognized reply id")]
    UnrecognizedReplyId(u64),

    #[error("Missing expected data")]
    DataMissing,
}