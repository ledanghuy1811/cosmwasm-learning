use cosmwasm_std::{DepsMut, MessageInfo, Response, ensure};

use crate::error::ContractError;
use crate::state::{PARENT, NAME};

pub fn change_name(deps: DepsMut, info: MessageInfo, name: String) -> Result<Response, ContractError> {
    let parent = PARENT.load(deps.storage)?;
    ensure!(parent == info.sender, ContractError::Unauthorized);

    NAME.save(deps.storage, &name)?;

    let resp = Response::new()
        .add_attribute("action", "change name")
        .add_attribute("sender", info.sender.to_string());

    Ok(resp)
}