use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const NAME: Item<String> = Item::new("name");
pub const COUNT_CHILD: Item<u64> = Item::new("count_child");
pub const CHILDREN: Map<u64, Addr> = Map::new("children");
