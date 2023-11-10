use cosmwasm_std::{Deps, StdResult};

use crate::msg::{ChildNameResp, ParentNameResp, QueryMsg};
use crate::state::{NAME, PARENT};

pub fn get_child_name(deps: Deps) -> StdResult<ChildNameResp> {
    let name = NAME.load(deps.storage)?;

    Ok(ChildNameResp { name })
}

pub fn get_parent_name(deps: Deps) -> StdResult<ParentNameResp> {
    let parent_addr = PARENT.load(deps.storage)?;

    let resp: ParentNameResp = deps
        .querier
        .query_wasm_smart(parent_addr, &QueryMsg::ParentName {})?;

    Ok(resp)
}
