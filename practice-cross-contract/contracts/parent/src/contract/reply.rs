use cosmwasm_std::{Addr, DepsMut, Response, StdError, SubMsgResponse};
use cw_utils::parse_instantiate_response_data;

use crate::error::ContractError;
use crate::state::{CHILDREN, COUNT_CHILD};

pub fn add_child_reply(
    deps: DepsMut,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    let data = response.data.ok_or(ContractError::DataMissing)?;
    let response = parse_instantiate_response_data(&data)?;
    let addr = Addr::unchecked(response.contract_address);

    let count_child = COUNT_CHILD.load(deps.storage)? + 1;
    CHILDREN.save(deps.storage, count_child, &addr)?;
    COUNT_CHILD.save(deps.storage, &count_child)?;

    let resp = Response::new()
        .add_attribute("action", "reply add child")
        .add_attribute("child addr", addr.to_string());

    Ok(resp)
}

pub fn change_child_name_reply(
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    if let Some(data) = response.data {
        let resp = Response::new().set_data(data);
        Ok(resp)
    } else {
        Ok(Response::new())
    }
}
