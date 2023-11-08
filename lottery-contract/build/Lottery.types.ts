export type Uint128 = string;
export type Addr = string;
export interface InstantiateMsg {
  minimal_donation: Coin;
  player: Addr[];
}
export interface Coin {
  amount: Uint128;
  denom: string;
}
export type ExecuteMsg = {
  donate: {};
} | {
  pick_winner: {};
};
export type QueryMsg = {
  contract_balance: {};
} | {
  minimal_donation: {};
};
export interface BalanceResp {
  balance: Coin;
}
export interface MinimalDonationResp {
  donation: Coin;
}