use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Reply};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, ExecuteMsg::*, InstantiateMsg, QueryMsg, QueryMsg::*};
use crate::state::{COUNT_CHILD, NAME};

mod execute;
mod query;
mod reply;

const CHILDREN_INSTANTIATION_REPLY_ID: u64 = 1;
const CHILDREN_CHANGE_NAME_REPLY_ID: u64 = 2;


pub fn instantiate(deps: DepsMut, msg: InstantiateMsg) -> StdResult<Response> {
    NAME.save(deps.storage, &msg.name)?;
    COUNT_CHILD.save(deps.storage, &1)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        Name {} => to_json_binary(&query::get_name(deps)?),
        Children { index } => to_json_binary(&query::get_children(deps, index)?),
    }
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        AddChildren {name} => execute::add_children(deps, env, info, name),
        ChangeChildName { child_index, name } => {
            execute::change_child_name(deps, info, child_index, name)
        }
    }
}

pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        CHILDREN_INSTANTIATION_REPLY_ID => reply::add_child_reply(deps, reply.result.into()),
        CHILDREN_CHANGE_NAME_REPLY_ID => reply::change_child_name_reply(reply.result.into()),
        id => Err(ContractError::UnrecognizedReplyId(id))
    }
}
