use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub parent: Addr,
    pub name: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeName { name: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ChildNameResp)]
    ChildName {},

    #[returns(ParentNameResp)]
    ParentName {},
}

#[cw_serde]
pub struct ChildNameResp {
    pub name: String,
}

#[cw_serde]
pub struct ParentNameResp {
    pub name: String,
}
