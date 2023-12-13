import { UncheckedDenom, UncheckedDepositInfo, Empty, ProposeMessage, Status, Addr, CheckedDepositInfo } from "./types";
export interface InstantiateMsg {
    deposit_info?: UncheckedDepositInfo | null;
    extension: Empty;
    open_proposal_submission: boolean;
}
export type ExecuteMsg = {
    propose: {
        msg: ProposeMessage;
    };
} | {
    update_config: {
        deposit_info?: UncheckedDepositInfo | null;
        open_proposal_submission: boolean;
    };
} | {
    withdraw: {
        denom?: UncheckedDenom | null;
    };
} | {
    extension: {
        msg: Empty;
    };
} | {
    add_proposal_submitted_hook: {
        address: string;
    };
} | {
    remove_proposal_submitted_hook: {
        address: string;
    };
} | {
    proposal_completed_hook: {
        new_status: Status;
        proposal_id: number;
    };
};
export type QueryMsg = {
    proposal_module: {};
} | {
    dao: {};
} | {
    config: {};
} | {
    deposit_info: {
        proposal_id: number;
    };
} | {
    proposal_submitted_hooks: {};
} | {
    query_extension: {
        msg: Empty;
    };
};
export interface Config {
    deposit_info?: CheckedDepositInfo | null;
    open_proposal_submission: boolean;
}
export interface DepositInfoResponse {
    deposit_info?: CheckedDepositInfo | null;
    proposer: Addr;
}
export interface HooksResponse {
    hooks: string[];
}
