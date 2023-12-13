export type Duration = {
    height: number;
} | {
    time: number;
};
export type Uint128 = string;
export type Binary = string;
export type Action = {
    transfer_ownership: {
        expiry?: Expiration | null;
        new_owner: string;
    };
} | "accept_ownership" | "renounce_ownership";
export type Expiration = {
    at_height: number;
} | {
    at_time: Timestamp;
} | {
    never: {};
};
export type Timestamp = Uint64;
export type Uint64 = string;
export interface Cw20ReceiveMsg {
    amount: Uint128;
    msg: Binary;
    sender: string;
}
export interface Claim {
    amount: Uint128;
    release_at: Expiration;
}
export type Addr = string;
export interface OwnershipForAddr {
    owner?: Addr | null;
    pending_expiry?: Expiration | null;
    pending_owner?: Addr | null;
}
export interface MetadataExt {
    role?: string | null;
    weight: number;
}
export type ActiveThreshold = {
    absolute_count: {
        count: Uint128;
    };
} | {
    percentage: {
        percent: Decimal;
    };
};
export type Decimal = string;
export type StakingInfo = {
    existing: {
        staking_contract_address: string;
    };
} | {
    new: {
        staking_code_id: number;
        unstaking_duration?: Duration | null;
    };
};
export type Logo = {
    url: string;
} | {
    embedded: EmbeddedLogo;
};
export type EmbeddedLogo = {
    svg: Binary;
} | {
    png: Binary;
};
export interface Cw20Coin {
    address: string;
    amount: Uint128;
}
export interface InstantiateMarketingInfo {
    description?: string | null;
    logo?: Logo | null;
    marketing?: string | null;
    project?: string | null;
}
export interface ContractVersion {
    contract: string;
    version: string;
}
export type Boolean = boolean;
export type UncheckedDenom = {
    native: string;
} | {
    cw20: string;
};
export type Schedule = "saturating_linear" | {
    piecewise_linear: [number, Uint128][];
};
export interface DenomUnit {
    aliases: string[];
    denom: string;
    exponent: number;
}
export interface Coin {
    amount: Uint128;
    denom: string;
}
export type CheckedDenom = {
    native: string;
} | {
    cw20: Addr;
};
export type Admin = {
    address: {
        addr: string;
    };
} | {
    core_module: {};
};
export interface ModuleInstantiateInfo {
    admin?: Admin | null;
    code_id: number;
    label: string;
    msg: Binary;
}
export type CosmosMsgForEmpty = {
    bank: BankMsg;
} | {
    custom: Empty;
} | {
    staking: StakingMsg;
} | {
    distribution: DistributionMsg;
} | {
    stargate: {
        type_url: string;
        value: Binary;
    };
} | {
    ibc: IbcMsg;
} | {
    wasm: WasmMsg;
} | {
    gov: GovMsg;
};
export type BankMsg = {
    send: {
        amount: Coin[];
        to_address: string;
    };
} | {
    burn: {
        amount: Coin[];
    };
};
export type StakingMsg = {
    delegate: {
        amount: Coin;
        validator: string;
    };
} | {
    undelegate: {
        amount: Coin;
        validator: string;
    };
} | {
    redelegate: {
        amount: Coin;
        dst_validator: string;
        src_validator: string;
    };
};
export type DistributionMsg = {
    set_withdraw_address: {
        address: string;
    };
} | {
    withdraw_delegator_reward: {
        validator: string;
    };
};
export type IbcMsg = {
    transfer: {
        amount: Coin;
        channel_id: string;
        timeout: IbcTimeout;
        to_address: string;
    };
} | {
    send_packet: {
        channel_id: string;
        data: Binary;
        timeout: IbcTimeout;
    };
} | {
    close_channel: {
        channel_id: string;
    };
};
export type WasmMsg = {
    execute: {
        contract_addr: string;
        funds: Coin[];
        msg: Binary;
    };
} | {
    instantiate: {
        admin?: string | null;
        code_id: number;
        funds: Coin[];
        label: string;
        msg: Binary;
    };
} | {
    migrate: {
        contract_addr: string;
        msg: Binary;
        new_code_id: number;
    };
} | {
    update_admin: {
        admin: string;
        contract_addr: string;
    };
} | {
    clear_admin: {
        contract_addr: string;
    };
};
export type GovMsg = {
    vote: {
        proposal_id: number;
        vote: VoteOption;
    };
};
export type VoteOption = "yes" | "no" | "abstain" | "no_with_veto";
export interface Empty {
}
export interface IbcTimeout {
    block?: IbcTimeoutBlock | null;
    timestamp?: Timestamp | null;
}
export interface IbcTimeoutBlock {
    height: number;
    revision: number;
}
export interface Cw721ReceiveMsg {
    msg: Binary;
    sender: string;
    token_id: string;
}
export type PreProposeInfo = {
    anyone_may_propose: {};
} | {
    module_may_propose: {
        info: ModuleInstantiateInfo;
    };
};
export type ArrayOfString = string[];
export type Threshold = {
    absolute_percentage: {
        percentage: PercentageThreshold;
    };
} | {
    threshold_quorum: {
        quorum: PercentageThreshold;
        threshold: PercentageThreshold;
    };
} | {
    absolute_count: {
        threshold: Uint128;
    };
};
export type PercentageThreshold = {
    majority: {};
} | {
    percent: Decimal;
};
export type Vote = "yes" | "no" | "abstain";
export interface SingleChoiceProposeMsg {
    description: string;
    msgs: CosmosMsgForEmpty[];
    proposer?: string | null;
    title: string;
}
export interface SingleChoiceProposal {
    allow_revoting: boolean;
    description: string;
    expiration: Expiration;
    min_voting_period?: Expiration | null;
    msgs: CosmosMsgForEmpty[];
    proposer: Addr;
    start_height: number;
    status: Status;
    threshold: Threshold;
    title: string;
    total_power: Uint128;
    votes: Votes;
}
export interface Votes {
    abstain: Uint128;
    no: Uint128;
    yes: Uint128;
}
export type ProposalCreationPolicy = {
    anyone: {};
} | {
    module: {
        addr: Addr;
    };
};
export type DepositToken = {
    token: {
        denom: UncheckedDenom;
    };
} | {
    voting_module_token: {};
};
export type DepositRefundPolicy = "always" | "only_passed" | "never";
export interface UncheckedDepositInfo {
    amount: Uint128;
    denom: DepositToken;
    refund_policy: DepositRefundPolicy;
}
export interface CheckedDepositInfo {
    amount: Uint128;
    denom: CheckedDenom;
    refund_policy: DepositRefundPolicy;
}
export interface MultipleChoiceOptions {
    options: MultipleChoiceOption[];
}
export interface MultipleChoiceOption {
    description: string;
    msgs: CosmosMsgForEmpty[];
    title: string;
}
export type ProposeMessage = {
    propose: {
        description: string;
        msgs: CosmosMsgForEmpty[];
        title: string;
    };
};
export type Status = "open" | "rejected" | "passed" | "executed" | "closed" | "execution_failed";
export interface VoteInfo {
    power: Uint128;
    rationale?: string | null;
    vote: Vote;
    voter: Addr;
}
export type { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
