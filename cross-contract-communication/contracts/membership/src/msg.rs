use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use common::msg::ProposeMemberData;
pub use common::msg::membership::ExecMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub starting_weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub halftime: u64,
    pub minimal_acceptances: u64,
    pub proxy_code_id: u64,
    pub distribution_code_id: u64,
    pub initial_members: Vec<String>,
}

#[cw_serde]
pub struct InstantationData {
    pub members: Vec<ProposeMemberData>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IsMemberResp)]
    IsMember { addr: String },
}

#[cw_serde]
pub struct IsMemberResp {
    pub is_member: bool,
}