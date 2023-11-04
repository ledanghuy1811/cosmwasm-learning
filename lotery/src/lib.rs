use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};

use msg::{InstantiateMsg, QueryMsg, ExecMsg};
use error::ContractError;

pub mod state;
pub mod msg;
pub mod contract;
pub mod error;
#[cfg(test)]
mod tests;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate (deps: DepsMut, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> Result<Response, ContractError> {
    use contract::exec;
    use msg::ExecMsg::*;

    match msg {
        Donate {} => exec::donate(deps, info),
        PickWinner {} => exec::pick_winner(deps, env, info)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        ContractBalance {} => to_json_binary(&query::contract_balance(deps, env)?),
        MinimalDonation {} => to_json_binary(&query::minimal_donate(deps)?)
    }
}
