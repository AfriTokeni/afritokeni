import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AgentBalance {
  'principal' : Principal,
  'total_deposits' : bigint,
  'total_commission_owed' : bigint,
  'total_commission_paid' : bigint,
  'last_settlement_date' : [] | [bigint],
}
export interface ConfirmDepositRequest {
  'deposit_code' : string,
  'agent_principal' : Principal,
}
export interface CreateDepositRequest {
  'user_principal' : Principal,
  'agent_principal' : Principal,
  'amount_ugx' : bigint,
}
export interface DepositTransaction {
  'id' : bigint,
  'status' : TransactionStatus,
  'user_principal' : Principal,
  'deposit_code' : string,
  'agent_principal' : Principal,
  'amount_ugx' : bigint,
  'timestamp' : bigint,
  'commission_ugx' : bigint,
}
export interface MonthlySettlement {
  'month' : string,
  'paid' : boolean,
  'agent_principal' : Principal,
  'total_commission' : bigint,
  'paid_date' : [] | [bigint],
}
export type Result = { 'Ok' : DepositTransaction } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Array<MonthlySettlement> } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : string };
export type TransactionStatus = { 'Confirmed' : null } |
  { 'Cancelled' : null } |
  { 'Pending' : null };
export interface _SERVICE {
  'confirm_deposit' : ActorMethod<[ConfirmDepositRequest], Result>,
  'create_deposit_request' : ActorMethod<[CreateDepositRequest], Result>,
  'create_monthly_settlement' : ActorMethod<[string], Result_1>,
  'get_agent_balance' : ActorMethod<[Principal], [] | [AgentBalance]>,
  'get_agent_deposits' : ActorMethod<[Principal], Array<DepositTransaction>>,
  'get_agent_settlements' : ActorMethod<[Principal], Array<MonthlySettlement>>,
  'get_all_agent_balances' : ActorMethod<[], Array<AgentBalance>>,
  'get_commission_rate' : ActorMethod<[], bigint>,
  'get_deposit' : ActorMethod<[bigint], [] | [DepositTransaction]>,
  'get_pending_deposits' : ActorMethod<[Principal], Array<DepositTransaction>>,
  'get_settlements_for_month' : ActorMethod<[string], Array<MonthlySettlement>>,
  'get_total_revenue' : ActorMethod<[], bigint>,
  'get_user_deposits' : ActorMethod<[Principal], Array<DepositTransaction>>,
  'mark_settlement_paid' : ActorMethod<[string, Principal], Result_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
