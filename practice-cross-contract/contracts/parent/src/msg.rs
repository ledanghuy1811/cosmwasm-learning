use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddChildren {name: String},
    ChangeChildName { child_index: u64, name: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NameReps)]
    Name {},
    #[returns(ChildrenResp)]
    Children { index: u64 },
}

#[cw_serde]
pub struct NameReps {
    pub name: String,
}

#[cw_serde]
pub struct ChildrenResp {
    pub children: Addr,
}
