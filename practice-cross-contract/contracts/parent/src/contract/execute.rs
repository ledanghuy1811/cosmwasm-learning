use cosmwasm_std::{to_json_binary, DepsMut, Env, MessageInfo, Response, WasmMsg, SubMsg};

use crate::{error::ContractError, state::CHILDREN};
use crate::state::COUNT_CHILD;
use children::msg::{InstantiateMsg as ChildrenInstantiateMsg, ExecuteMsg };

use super::{CHILDREN_INSTANTIATION_REPLY_ID, CHILDREN_CHANGE_NAME_REPLY_ID};

pub fn add_children(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let count_child = COUNT_CHILD.load(deps.storage)?;

    let init_msg = ChildrenInstantiateMsg {
        parent: env.clone().contract.address,
        name,
    };
    let init_msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: count_child,
        msg: to_json_binary(&init_msg)?,
        funds: vec![],
        label: format!("Children {}", count_child),
    };
    let init_msg = SubMsg::reply_on_success(init_msg, CHILDREN_INSTANTIATION_REPLY_ID);

    let resp = Response::new()
        .add_submessage(init_msg)
        .add_attribute("action", "add_children")
        .add_attribute("sender", info.sender.to_string());

    Ok(resp)
}

pub fn change_child_name(
    deps: DepsMut,
    info: MessageInfo,
    child_index: u64,
    name: String,
) -> Result<Response, ContractError> {
    let child_addr = CHILDREN.load(deps.storage, child_index)?;

    let change_name_msg = ExecuteMsg::ChangeName { name };
    let change_name_msg = WasmMsg::Execute { 
        contract_addr: child_addr.to_string(),
        msg: to_json_binary(&change_name_msg)?,
        funds: vec![]
    };
    let change_name_msg = SubMsg::reply_on_success(change_name_msg, CHILDREN_CHANGE_NAME_REPLY_ID);

    let resp = Response::new()
        .add_submessage(change_name_msg)
        .add_attribute("action", "change child name")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("child", child_addr.to_string());

    Ok(resp)
}
