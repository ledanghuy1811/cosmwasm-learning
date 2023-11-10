use thiserror::Error;
use cosmwasm_std::StdError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized
}