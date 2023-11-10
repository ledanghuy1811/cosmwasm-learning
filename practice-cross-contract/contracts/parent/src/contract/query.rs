use cosmwasm_std::{Deps, StdResult};

use crate::msg::{ChildrenResp, NameReps};
use crate::state::{CHILDREN, NAME};

pub fn get_name(deps: Deps) -> StdResult<NameReps> {
    let name = NAME.load(deps.storage)?;

    Ok(NameReps { name })
}

pub fn get_children(deps: Deps, index: u64) -> StdResult<ChildrenResp> {
    let children = CHILDREN.load(deps.storage, index)?;

    Ok(ChildrenResp { children })
}
