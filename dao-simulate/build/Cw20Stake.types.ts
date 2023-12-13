import { Duration, Uint128, Action, Cw20ReceiveMsg, Claim, Addr } from "./types";
export interface InstantiateMsg {
    owner?: string | null;
    token_address: string;
    unstaking_duration?: Duration | null;
}
export type ExecuteMsg = {
    receive: Cw20ReceiveMsg;
} | {
    unstake: {
        amount: Uint128;
    };
} | {
    claim: {};
} | {
    update_config: {
        duration?: Duration | null;
    };
} | {
    add_hook: {
        addr: string;
    };
} | {
    remove_hook: {
        addr: string;
    };
} | {
    update_ownership: Action;
};
export type QueryMsg = {
    staked_balance_at_height: {
        address: string;
        height?: number | null;
    };
} | {
    total_staked_at_height: {
        height?: number | null;
    };
} | {
    staked_value: {
        address: string;
    };
} | {
    total_value: {};
} | {
    get_config: {};
} | {
    claims: {
        address: string;
    };
} | {
    get_hooks: {};
} | {
    list_stakers: {
        limit?: number | null;
        start_after?: string | null;
    };
} | {
    ownership: {};
};
export type MigrateMsg = {
    from_v1: {};
};
export interface ClaimsResponse {
    claims: Claim[];
}
export interface Config {
    token_address: Addr;
    unstaking_duration?: Duration | null;
}
export interface GetHooksResponse {
    hooks: string[];
}
export interface ListStakersResponse {
    stakers: StakerBalanceResponse[];
}
export interface StakerBalanceResponse {
    address: string;
    balance: Uint128;
}
export interface StakedBalanceAtHeightResponse {
    balance: Uint128;
    height: number;
}
export interface StakedValueResponse {
    value: Uint128;
}
export interface TotalStakedAtHeightResponse {
    height: number;
    total: Uint128;
}
export interface TotalValueResponse {
    total: Uint128;
}
