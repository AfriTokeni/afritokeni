import type { Principal } from "@dfinity/principal";
import type { ActorMethod } from "@dfinity/agent";
import type { IDL } from "@dfinity/candid";

export interface AgentBalance {
  principal: Principal;
  total_deposits: bigint;
  total_commission_owed: bigint;
  total_commission_paid: bigint;
  last_settlement_date: [] | [bigint];
}
export interface ConfirmDepositRequest {
  deposit_code: string;
  agent_principal: Principal;
}
export interface CreateDepositRequest {
  user_principal: Principal;
  agent_principal: Principal;
  amount_ugx: bigint;
}
export interface DepositTransaction {
  id: bigint;
  status: TransactionStatus;
  user_principal: Principal;
  deposit_code: string;
  agent_principal: Principal;
  amount_ugx: bigint;
  timestamp: bigint;
  commission_ugx: bigint;
}
export interface MonthlySettlement {
  month: string;
  paid: boolean;
  agent_principal: Principal;
  total_commission: bigint;
  paid_date: [] | [bigint];
}
export type Result = { Ok: DepositTransaction } | { Err: string };
export type Result_1 = { Ok: Array<MonthlySettlement> } | { Err: string };
export type Result_2 = { Ok: null } | { Err: string };
export type TransactionStatus =
  | { Confirmed: null }
  | { Cancelled: null }
  | { Pending: null };
export interface _SERVICE {
  confirm_deposit: ActorMethod<[ConfirmDepositRequest], Result>;
  create_deposit_request: ActorMethod<[CreateDepositRequest], Result>;
  create_monthly_settlement: ActorMethod<[string], Result_1>;
  get_agent_balance: ActorMethod<[Principal], [] | [AgentBalance]>;
  get_agent_deposits: ActorMethod<[Principal], Array<DepositTransaction>>;
  get_agent_settlements: ActorMethod<[Principal], Array<MonthlySettlement>>;
  get_all_agent_balances: ActorMethod<[], Array<AgentBalance>>;
  get_commission_rate: ActorMethod<[], bigint>;
  get_deposit: ActorMethod<[bigint], [] | [DepositTransaction]>;
  get_pending_deposits: ActorMethod<[Principal], Array<DepositTransaction>>;
  get_settlements_for_month: ActorMethod<[string], Array<MonthlySettlement>>;
  get_total_revenue: ActorMethod<[], bigint>;
  get_user_deposits: ActorMethod<[Principal], Array<DepositTransaction>>;
  mark_settlement_paid: ActorMethod<[string, Principal], Result_2>;
}
export const idlFactory: IDL.InterfaceFactory = ({ IDL }) => {
  const TransactionStatus = IDL.Variant({
    Confirmed: IDL.Null,
    Cancelled: IDL.Null,
    Pending: IDL.Null,
  });
  const DepositTransaction = IDL.Record({
    id: IDL.Nat64,
    status: TransactionStatus,
    user_principal: IDL.Principal,
    deposit_code: IDL.Text,
    agent_principal: IDL.Principal,
    amount_ugx: IDL.Nat64,
    timestamp: IDL.Nat64,
    commission_ugx: IDL.Nat64,
  });
  const Result = IDL.Variant({ Ok: DepositTransaction, Err: IDL.Text });
  const ConfirmDepositRequest = IDL.Record({
    deposit_code: IDL.Text,
    agent_principal: IDL.Principal,
  });
  const CreateDepositRequest = IDL.Record({
    user_principal: IDL.Principal,
    agent_principal: IDL.Principal,
    amount_ugx: IDL.Nat64,
  });
  const MonthlySettlement = IDL.Record({
    month: IDL.Text,
    paid: IDL.Bool,
    agent_principal: IDL.Principal,
    total_commission: IDL.Nat64,
    paid_date: IDL.Opt(IDL.Nat64),
  });
  const Result_1 = IDL.Variant({
    Ok: IDL.Vec(MonthlySettlement),
    Err: IDL.Text,
  });
  const AgentBalance = IDL.Record({
    principal: IDL.Principal,
    total_deposits: IDL.Nat64,
    total_commission_owed: IDL.Nat64,
    total_commission_paid: IDL.Nat64,
    last_settlement_date: IDL.Opt(IDL.Nat64),
  });
  const Result_2 = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });
  return IDL.Service({
    confirm_deposit: IDL.Func([ConfirmDepositRequest], [Result], []),
    create_deposit_request: IDL.Func([CreateDepositRequest], [Result], []),
    create_monthly_settlement: IDL.Func([IDL.Text], [Result_1], []),
    get_agent_balance: IDL.Func(
      [IDL.Principal],
      [IDL.Opt(AgentBalance)],
      ["query"],
    ),
    get_agent_deposits: IDL.Func(
      [IDL.Principal],
      [IDL.Vec(DepositTransaction)],
      ["query"],
    ),
    get_agent_settlements: IDL.Func(
      [IDL.Principal],
      [IDL.Vec(MonthlySettlement)],
      ["query"],
    ),
    get_all_agent_balances: IDL.Func([], [IDL.Vec(AgentBalance)], ["query"]),
    get_commission_rate: IDL.Func([], [IDL.Nat64], ["query"]),
    get_deposit: IDL.Func(
      [IDL.Nat64],
      [IDL.Opt(DepositTransaction)],
      ["query"],
    ),
    get_pending_deposits: IDL.Func(
      [IDL.Principal],
      [IDL.Vec(DepositTransaction)],
      ["query"],
    ),
    get_settlements_for_month: IDL.Func(
      [IDL.Text],
      [IDL.Vec(MonthlySettlement)],
      ["query"],
    ),
    get_total_revenue: IDL.Func([], [IDL.Nat64], ["query"]),
    get_user_deposits: IDL.Func(
      [IDL.Principal],
      [IDL.Vec(DepositTransaction)],
      ["query"],
    ),
    mark_settlement_paid: IDL.Func([IDL.Text, IDL.Principal], [Result_2], []),
  });
};
export const init: (args: { IDL: typeof IDL }) => IDL.Type[] = ({
  IDL: _IDL,
}) => {
  return [];
};
