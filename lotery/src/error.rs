use cosmwasm_std::{Coin, StdError, Addr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized: only {owner} can call it")]
    Unauthorized { owner: String },

    #[error("Not enough token or different denom")]
    NotEnoughTokenOrDiffDenom { donate: Coin },

    #[error("Not enough players")]
    NotEnoughPlayers {players: Vec<Addr>}
}
