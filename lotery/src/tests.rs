use cosmwasm_std::{coin, coins, Addr};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::error::ContractError;
use crate::msg::{BalanceResp, ExecMsg, InstantiateMsg, QueryMsg};
use crate::{execute, instantiate, query};

#[test]
pub fn balance_query() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("manager"),
            &InstantiateMsg {
                player: vec![],
                minimal_donation: coin(5, "atom"),
            },
            &[],
            "Lotery contract",
            None,
        )
        .unwrap();
    let resp: BalanceResp = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::ContractBalance {})
        .unwrap();

    assert_eq!(
        resp,
        BalanceResp {
            balance: coin(0, "atom")
        }
    );
}

#[test]
pub fn donation_with_lower_token() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(10, "atom"))
            .unwrap()
    });
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("manager"),
            &InstantiateMsg {
                player: vec![],
                minimal_donation: coin(5, "atom"),
            },
            &[],
            "Lotery contract",
            None,
        )
        .unwrap();

    let donation_coin = coin(3, "atom");
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecMsg::Donate {},
            &[donation_coin.clone()],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::NotEnoughTokenOrDiffDenom {
            donate: donation_coin
        },
        err.downcast().unwrap()
    )
}

#[test]
pub fn donation() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(10, "atom"))
            .unwrap()
    });
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("manager"),
            &InstantiateMsg {
                player: vec![],
                minimal_donation: coin(5, "atom"),
            },
            &[],
            "Lotery contract",
            None,
        )
        .unwrap();

    let donation_coin = coin(10, "atom");
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecMsg::Donate {},
        &[donation_coin.clone()],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "atom")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance(addr, "atom")
            .unwrap()
            .amount
            .u128(),
        10
    );
}

#[test]
pub fn pick_winner_unauthorized() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(10, "atom"))
            .unwrap()
    });
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("manager"),
            &InstantiateMsg {
                player: vec![],
                minimal_donation: coin(5, "atom"),
            },
            &[],
            "Lotery contract",
            None,
        )
        .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecMsg::PickWinner {},
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            owner: "user".to_string()
        },
        err.downcast().unwrap()
    );
}

#[test]
pub fn pick_winner_not_enough_players() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(10, "atom"))
            .unwrap()
    });
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("manager"),
            &InstantiateMsg {
                player: vec![],
                minimal_donation: coin(5, "atom"),
            },
            &[],
            "Lotery contract",
            None,
        )
        .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("manager"),
            addr.clone(),
            &ExecMsg::PickWinner {},
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::NotEnoughPlayers { players: (vec![]) },
        err.downcast().unwrap()
    );
}

#[test]
pub fn pick_winner() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user1"), coins(10, "atom"))
            .unwrap()
    });
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    app.send_tokens(
        Addr::unchecked("user1"),
        Addr::unchecked("user2"),
        &coins(3, "atom"),
    )
    .unwrap();
    app.send_tokens(
        Addr::unchecked("user1"),
        Addr::unchecked("user3"),
        &coins(3, "atom"),
    )
    .unwrap();

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("manager"),
            &InstantiateMsg {
                player: vec![],
                minimal_donation: coin(3, "atom"),
            },
            &[],
            "Lotery contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("user1"),
        addr.clone(),
        &ExecMsg::Donate {},
        &[coin(4, "atom")],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("user2"),
        addr.clone(),
        &ExecMsg::Donate {},
        &[coin(3, "atom")],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked("user3"),
        addr.clone(),
        &ExecMsg::Donate {},
        &[coin(3, "atom")],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("manager"),
        addr.clone(),
        &ExecMsg::PickWinner {},
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "atom")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance("manager", "atom")
            .unwrap()
            .amount
            .u128(),
        1
    );

    let balance_user1 = app
        .wrap()
        .query_balance("user1", "atom")
        .unwrap()
        .amount
        .u128();
    if balance_user1 > 0 {
        assert_eq!(balance_user1, 9);
    }
    let balance_user2 = app
        .wrap()
        .query_balance("user2", "atom")
        .unwrap()
        .amount
        .u128();
    if balance_user2 > 0 {
        assert_eq!(balance_user2, 9);
    }
    let balance_user3 = app
        .wrap()
        .query_balance("user3", "atom")
        .unwrap()
        .amount
        .u128();
    if balance_user3 > 0 {
        assert_eq!(balance_user3, 9);
    }
}
