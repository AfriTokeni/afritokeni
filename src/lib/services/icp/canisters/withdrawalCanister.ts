import type { Principal } from "@dfinity/principal";
import type { ActorMethod } from "@dfinity/agent";
import type { IDL } from "@dfinity/candid";

export interface AgentEarnings {
  principal: Principal;
  total_withdrawals_processed: bigint;
  last_withdrawal_date: [] | [bigint];
  total_fees_earned: bigint;
  total_fees_withdrawn: bigint;
}
export interface ConfirmWithdrawalRequest {
  agent_principal: Principal;
  withdrawal_code: string;
}
export interface CreateWithdrawalRequest {
  user_principal: Principal;
  agent_principal: Principal;
  amount_ugx: bigint;
}
export type Result = { Ok: WithdrawalTransaction } | { Err: string };
export type TransactionStatus =
  | { Confirmed: null }
  | { Cancelled: null }
  | { Pending: null };
export interface WithdrawalTransaction {
  id: bigint;
  status: TransactionStatus;
  user_principal: Principal;
  agent_principal: Principal;
  platform_fee_ugx: bigint;
  amount_ugx: bigint;
  agent_fee_ugx: bigint;
  timestamp: bigint;
  withdrawal_code: string;
}
export interface _SERVICE {
  confirm_withdrawal: ActorMethod<[ConfirmWithdrawalRequest], Result>;
  create_withdrawal_request: ActorMethod<[CreateWithdrawalRequest], Result>;
  get_agent_earnings: ActorMethod<[Principal], [] | [AgentEarnings]>;
  get_agent_withdrawals: ActorMethod<[Principal], Array<WithdrawalTransaction>>;
  get_all_agent_earnings: ActorMethod<[], Array<AgentEarnings>>;
  get_fee_split: ActorMethod<[], [bigint, bigint]>;
  get_pending_withdrawals: ActorMethod<
    [Principal],
    Array<WithdrawalTransaction>
  >;
  get_total_agent_earnings: ActorMethod<[], bigint>;
  get_total_platform_revenue: ActorMethod<[], bigint>;
  get_user_withdrawals: ActorMethod<[Principal], Array<WithdrawalTransaction>>;
  get_withdrawal: ActorMethod<[bigint], [] | [WithdrawalTransaction]>;
}
export const idlFactory: IDL.InterfaceFactory = ({ IDL }) => {
  const TransactionStatus = IDL.Variant({
    Confirmed: IDL.Null,
    Cancelled: IDL.Null,
    Pending: IDL.Null,
  });
  const WithdrawalTransaction = IDL.Record({
    id: IDL.Nat64,
    status: TransactionStatus,
    user_principal: IDL.Principal,
    agent_principal: IDL.Principal,
    platform_fee_ugx: IDL.Nat64,
    amount_ugx: IDL.Nat64,
    agent_fee_ugx: IDL.Nat64,
    timestamp: IDL.Nat64,
    withdrawal_code: IDL.Text,
  });
  const Result = IDL.Variant({ Ok: WithdrawalTransaction, Err: IDL.Text });
  const ConfirmWithdrawalRequest = IDL.Record({
    agent_principal: IDL.Principal,
    withdrawal_code: IDL.Text,
  });
  const CreateWithdrawalRequest = IDL.Record({
    user_principal: IDL.Principal,
    agent_principal: IDL.Principal,
    amount_ugx: IDL.Nat64,
  });
  const AgentEarnings = IDL.Record({
    principal: IDL.Principal,
    total_withdrawals_processed: IDL.Nat64,
    last_withdrawal_date: IDL.Opt(IDL.Nat64),
    total_fees_earned: IDL.Nat64,
    total_fees_withdrawn: IDL.Nat64,
  });
  return IDL.Service({
    confirm_withdrawal: IDL.Func([ConfirmWithdrawalRequest], [Result], []),
    create_withdrawal_request: IDL.Func(
      [CreateWithdrawalRequest],
      [Result],
      [],
    ),
    get_agent_earnings: IDL.Func(
      [IDL.Principal],
      [IDL.Opt(AgentEarnings)],
      ["query"],
    ),
    get_agent_withdrawals: IDL.Func(
      [IDL.Principal],
      [IDL.Vec(WithdrawalTransaction)],
      ["query"],
    ),
    get_all_agent_earnings: IDL.Func([], [IDL.Vec(AgentEarnings)], ["query"]),
    get_fee_split: IDL.Func([], [IDL.Nat64, IDL.Nat64], ["query"]),
    get_pending_withdrawals: IDL.Func(
      [IDL.Principal],
      [IDL.Vec(WithdrawalTransaction)],
      ["query"],
    ),
    get_total_agent_earnings: IDL.Func([], [IDL.Nat64], ["query"]),
    get_total_platform_revenue: IDL.Func([], [IDL.Nat64], ["query"]),
    get_user_withdrawals: IDL.Func(
      [IDL.Principal],
      [IDL.Vec(WithdrawalTransaction)],
      ["query"],
    ),
    get_withdrawal: IDL.Func(
      [IDL.Nat64],
      [IDL.Opt(WithdrawalTransaction)],
      ["query"],
    ),
  });
};
export const init: (args: { IDL: typeof IDL }) => IDL.Type[] = ({ IDL }) => {
  return [];
};
