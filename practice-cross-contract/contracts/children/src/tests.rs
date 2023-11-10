use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::msg::{ChildNameResp, InstantiateMsg, QueryMsg};
use crate::{execute, instantiate, query};

#[test]
pub fn get_child_name() {
    let mut app = App::default();
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("parent"),
            &InstantiateMsg {
                parent: Addr::unchecked("parent"),
                name: "children".to_string(),
            },
            &[],
            "Children Contract",
            None,
        )
        .unwrap();

    let resp: ChildNameResp = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::ChildName {})
        .unwrap();

    assert_eq!(
        resp,
        ChildNameResp {
            name: "children".to_string()
        }
    );
}
