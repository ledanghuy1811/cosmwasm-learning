import { Duration, PreProposeInfo, Threshold, Vote, SingleChoiceProposeMsg, Addr, VoteInfo, ContractVersion, SingleChoiceProposal } from "./types";
export interface InstantiateMsg {
    allow_revoting: boolean;
    close_proposal_on_execution_failure: boolean;
    max_voting_period: Duration;
    min_voting_period?: Duration | null;
    only_members_execute: boolean;
    pre_propose_info: PreProposeInfo;
    threshold: Threshold;
}
export type ExecuteMsg = {
    propose: SingleChoiceProposeMsg;
} | {
    vote: {
        proposal_id: number;
        rationale?: string | null;
        vote: Vote;
    };
} | {
    update_rationale: {
        proposal_id: number;
        rationale?: string | null;
    };
} | {
    execute: {
        proposal_id: number;
    };
} | {
    close: {
        proposal_id: number;
    };
} | {
    update_config: {
        allow_revoting: boolean;
        close_proposal_on_execution_failure: boolean;
        dao: string;
        max_voting_period: Duration;
        min_voting_period?: Duration | null;
        only_members_execute: boolean;
        threshold: Threshold;
    };
} | {
    update_pre_propose_info: {
        info: PreProposeInfo;
    };
} | {
    add_proposal_hook: {
        address: string;
    };
} | {
    remove_proposal_hook: {
        address: string;
    };
} | {
    add_vote_hook: {
        address: string;
    };
} | {
    remove_vote_hook: {
        address: string;
    };
};
export type QueryMsg = {
    config: {};
} | {
    proposal: {
        proposal_id: number;
    };
} | {
    list_proposals: {
        limit?: number | null;
        start_after?: number | null;
    };
} | {
    reverse_proposals: {
        limit?: number | null;
        start_before?: number | null;
    };
} | {
    get_vote: {
        proposal_id: number;
        voter: string;
    };
} | {
    list_votes: {
        limit?: number | null;
        proposal_id: number;
        start_after?: string | null;
    };
} | {
    proposal_count: {};
} | {
    proposal_creation_policy: {};
} | {
    proposal_hooks: {};
} | {
    vote_hooks: {};
} | {
    dao: {};
} | {
    info: {};
} | {
    next_proposal_id: {};
};
export type MigrateMsg = {
    from_v1: {
        close_proposal_on_execution_failure: boolean;
        pre_propose_info: PreProposeInfo;
    };
} | {
    from_compatible: {};
};
export interface Config {
    allow_revoting: boolean;
    close_proposal_on_execution_failure: boolean;
    dao: Addr;
    max_voting_period: Duration;
    min_voting_period?: Duration | null;
    only_members_execute: boolean;
    threshold: Threshold;
}
export interface VoteResponse {
    vote?: VoteInfo | null;
}
export interface InfoResponse {
    info: ContractVersion;
}
export interface ProposalListResponse {
    proposals: ProposalResponse[];
}
export interface ProposalResponse {
    id: number;
    proposal: SingleChoiceProposal;
}
export interface VoteListResponse {
    votes: VoteInfo[];
}
export interface HooksResponse {
    hooks: string[];
}
