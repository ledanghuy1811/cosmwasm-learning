use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

pub const MANAGER: Item<Addr> = Item::new("manager");
pub const PLAYERS: Item<Vec<Addr>> = Item::new("players");
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");