use cw_storage_plus::Item;
use cosmwasm_std::Addr;

pub const PARENT: Item<Addr> = Item::new("parent");
pub const NAME: Item<String> = Item::new("child_name");