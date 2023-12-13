import { ActiveThreshold, Uint128, StakingInfo, Duration, Cw20Coin, InstantiateMarketingInfo, ContractVersion } from "./types";
export type TokenInfo = {
    existing: {
        address: string;
        staking_contract: StakingInfo;
    };
} | {
    new: {
        code_id: number;
        decimals: number;
        initial_balances: Cw20Coin[];
        initial_dao_balance?: Uint128 | null;
        label: string;
        marketing?: InstantiateMarketingInfo | null;
        name: string;
        staking_code_id: number;
        symbol: string;
        unstaking_duration?: Duration | null;
    };
};
export interface InstantiateMsg {
    active_threshold?: ActiveThreshold | null;
    token_info: TokenInfo;
}
export type ExecuteMsg = {
    update_active_threshold: {
        new_threshold?: ActiveThreshold | null;
    };
};
export type QueryMsg = {
    staking_contract: {};
} | {
    active_threshold: {};
} | {
    voting_power_at_height: {
        address: string;
        height?: number | null;
    };
} | {
    total_power_at_height: {
        height?: number | null;
    };
} | {
    dao: {};
} | {
    info: {};
} | {
    token_contract: {};
} | {
    is_active: {};
};
export interface MigrateMsg {
}
export interface ActiveThresholdResponse {
    active_threshold?: ActiveThreshold | null;
}
export interface InfoResponse {
    info: ContractVersion;
}
export interface TotalPowerAtHeightResponse {
    height: number;
    power: Uint128;
}
export interface VotingPowerAtHeightResponse {
    height: number;
    power: Uint128;
}
