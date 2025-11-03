import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AgentEarnings {
  'principal' : Principal,
  'total_withdrawals_processed' : bigint,
  'last_withdrawal_date' : [] | [bigint],
  'total_fees_earned' : bigint,
  'total_fees_withdrawn' : bigint,
}
export interface ConfirmWithdrawalRequest {
  'agent_principal' : Principal,
  'withdrawal_code' : string,
}
export interface CreateWithdrawalRequest {
  'user_principal' : Principal,
  'agent_principal' : Principal,
  'amount_ugx' : bigint,
}
export type Result = { 'Ok' : WithdrawalTransaction } |
  { 'Err' : string };
export type TransactionStatus = { 'Confirmed' : null } |
  { 'Cancelled' : null } |
  { 'Pending' : null };
export interface WithdrawalTransaction {
  'id' : bigint,
  'status' : TransactionStatus,
  'user_principal' : Principal,
  'agent_principal' : Principal,
  'platform_fee_ugx' : bigint,
  'amount_ugx' : bigint,
  'agent_fee_ugx' : bigint,
  'timestamp' : bigint,
  'withdrawal_code' : string,
}
export interface _SERVICE {
  'confirm_withdrawal' : ActorMethod<[ConfirmWithdrawalRequest], Result>,
  'create_withdrawal_request' : ActorMethod<[CreateWithdrawalRequest], Result>,
  'get_agent_earnings' : ActorMethod<[Principal], [] | [AgentEarnings]>,
  'get_agent_withdrawals' : ActorMethod<
    [Principal],
    Array<WithdrawalTransaction>
  >,
  'get_all_agent_earnings' : ActorMethod<[], Array<AgentEarnings>>,
  'get_fee_split' : ActorMethod<[], [bigint, bigint]>,
  'get_pending_withdrawals' : ActorMethod<
    [Principal],
    Array<WithdrawalTransaction>
  >,
  'get_total_agent_earnings' : ActorMethod<[], bigint>,
  'get_total_platform_revenue' : ActorMethod<[], bigint>,
  'get_user_withdrawals' : ActorMethod<
    [Principal],
    Array<WithdrawalTransaction>
  >,
  'get_withdrawal' : ActorMethod<[bigint], [] | [WithdrawalTransaction]>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
