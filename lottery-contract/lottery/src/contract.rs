use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::msg::InstantiateMsg;
use crate::state::{MANAGER, MINIMAL_DONATION, PLAYERS};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    MANAGER.save(deps.storage, &info.sender)?;
    PLAYERS.save(deps.storage, &msg.player)?;
    MINIMAL_DONATION.save(deps.storage, &msg.minimal_donation)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, Env, StdResult};

    use crate::msg::{BalanceResp, MinimalDonationResp};
    use crate::state::MINIMAL_DONATION;

    pub fn contract_balance(deps: Deps, env: Env) -> StdResult<BalanceResp> {
        let denom = MINIMAL_DONATION.load(deps.storage)?.denom;
        let balance = deps.querier.query_balance(env.contract.address, denom)?;
        Ok(BalanceResp { balance: balance })
    }

    pub fn minimal_donate(deps: Deps) -> StdResult<MinimalDonationResp> {
        let min_donate = MINIMAL_DONATION.load(deps.storage)?;
        Ok(MinimalDonationResp {
            donation: min_donate,
        })
    }
}

pub mod exec {
    use std::vec;

    use cosmwasm_std::{coin, BankMsg, DepsMut, Env, Event, MessageInfo, Response};

    use crate::error::ContractError;
    use crate::state::{MANAGER, MINIMAL_DONATION, PLAYERS};

    use super::common_function::{check_owner, check_player_length, make_pseudo_random};

    pub fn donate(deps: DepsMut, mut info: MessageInfo) -> Result<Response, ContractError> {
        let mut current_players = PLAYERS.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;
        let sender_donation = info.funds.pop().unwrap();

        if sender_donation.denom == minimal_donation.denom
            && sender_donation.amount >= minimal_donation.amount
        {
            if !current_players.contains(&info.sender) {
                current_players.push(info.clone().sender);
                PLAYERS.save(deps.storage, &current_players)?;
            }
        } else {
            return Err(ContractError::NotEnoughTokenOrDiffDenom {
                donate: sender_donation,
            });
        }

        let event = Event::new("donate").add_attribute("address", &info.sender);
        let resp = Response::new()
            .add_event(event)
            .add_attribute("action", "donate for lotery")
            .add_attribute("added account", &info.sender);

        Ok(resp)
    }

    pub fn pick_winner(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let current_player = PLAYERS.load(deps.storage)?;
        if check_owner(&deps, &info)? != true {
            return Err(ContractError::Unauthorized {
                owner: info.sender.to_string(),
            });
        }
        if check_player_length(&deps)? != true {
            return Err(ContractError::NotEnoughPlayers {
                players: current_player,
            });
        }

        let denom = MINIMAL_DONATION.load(deps.storage)?.denom;
        let manager = MANAGER.load(deps.storage)?;
        let winner_index = make_pseudo_random(&env, current_player.len() as u64);
        let winner = &current_player[winner_index];
        let contract_balance = deps.querier.query_balance(env.contract.address, denom)?;

        let token_for_winner = contract_balance.amount.u128() * 9 / 10; // 90%
        let token_fee_manager = contract_balance.amount.u128() / 10;

        let send_token_winner = BankMsg::Send {
            to_address: winner.to_string(),
            amount: vec![coin(token_for_winner, contract_balance.clone().denom)],
        };
        let send_token_fee = BankMsg::Send {
            to_address: manager.to_string(),
            amount: vec![coin(token_fee_manager, contract_balance.clone().denom)],
        };

        let resp = Response::new()
            .add_messages(vec![send_token_winner, send_token_fee])
            .add_attribute("action", "pick winner")
            .add_attribute("winner", winner.to_string());

        Ok(resp)
    }
}

pub mod common_function {
    use crate::state::{MANAGER, PLAYERS};
    use cosmwasm_std::{DepsMut, Env, MessageInfo, StdResult};

    pub fn check_owner(deps: &DepsMut, info: &MessageInfo) -> StdResult<bool> {
        let owner = MANAGER.load(deps.storage)?;
        if info.sender != owner {
            return Ok(false);
        }
        Ok(true)
    }

    pub fn check_player_length(deps: &DepsMut) -> StdResult<bool> {
        let players = PLAYERS.load(deps.storage)?;
        if players.len() < 3 {
            return Ok(false);
        }
        Ok(true)
    }

    pub fn make_pseudo_random(env: &Env, num_player: u64) -> usize {
        let pseudo_number = env.block.time.seconds();
        let pseudo_random = pseudo_number % num_player;
        pseudo_random as usize
    }
}
