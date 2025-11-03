import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ExchangeRequest {
  'to_token' : Token,
  'from_token' : Token,
  'min_output' : bigint,
  'amount' : bigint,
}
export interface ExchangeResult {
  'tx_id' : string,
  'output_amount' : bigint,
  'spread_amount' : bigint,
}
export type Result = { 'Ok' : ExchangeResult } |
  { 'Err' : string };
export type Token = { 'CkUSDC' : null } |
  { 'CkBTC' : null };
export interface _SERVICE {
  'get_company_wallet' : ActorMethod<[], string>,
  'get_dex_provider' : ActorMethod<[], string>,
  'get_sonic_canister' : ActorMethod<[], string>,
  'get_spread_percentage' : ActorMethod<[], bigint>,
  'swap_tokens' : ActorMethod<[ExchangeRequest], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
