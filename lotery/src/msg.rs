use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub player: Vec<Addr>,
    pub minimal_donation: Coin,
}

#[cw_serde]
pub enum ExecMsg {
    Donate {},
    PickWinner {}
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalanceResp)]
    ContractBalance {},
    #[returns(MinimalDonationResp)]
    MinimalDonation {},
}

#[cw_serde]
pub struct BalanceResp {
    pub balance: Coin,
}

#[cw_serde]
pub struct MinimalDonationResp {
    pub donation: Coin,
}
