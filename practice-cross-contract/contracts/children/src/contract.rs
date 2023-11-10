use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, ExecuteMsg::*, InstantiateMsg, QueryMsg, QueryMsg::*};
use crate::state::{NAME, PARENT};

mod execute;
mod query;

pub fn instantiate(deps: DepsMut, msg: InstantiateMsg) -> StdResult<Response> {
    PARENT.save(deps.storage, &msg.parent)?;
    NAME.save(deps.storage, &msg.name)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        ChildName {} => to_json_binary(&query::get_child_name(deps)?),
        ParentName {} => to_json_binary(&query::get_parent_name(deps)?),
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ChangeName { name } => execute::change_name(deps, info, name),
    }
}
