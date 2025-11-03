export const idlFactory = ({ IDL }) => {
  const ConfirmDepositRequest = IDL.Record({
    'deposit_code' : IDL.Text,
    'agent_principal' : IDL.Principal,
  });
  const TransactionStatus = IDL.Variant({
    'Confirmed' : IDL.Null,
    'Cancelled' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const DepositTransaction = IDL.Record({
    'id' : IDL.Nat64,
    'status' : TransactionStatus,
    'user_principal' : IDL.Principal,
    'deposit_code' : IDL.Text,
    'agent_principal' : IDL.Principal,
    'amount_ugx' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
    'commission_ugx' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : DepositTransaction, 'Err' : IDL.Text });
  const CreateDepositRequest = IDL.Record({
    'user_principal' : IDL.Principal,
    'agent_principal' : IDL.Principal,
    'amount_ugx' : IDL.Nat64,
  });
  const MonthlySettlement = IDL.Record({
    'month' : IDL.Text,
    'paid' : IDL.Bool,
    'agent_principal' : IDL.Principal,
    'total_commission' : IDL.Nat64,
    'paid_date' : IDL.Opt(IDL.Nat64),
  });
  const Result_1 = IDL.Variant({
    'Ok' : IDL.Vec(MonthlySettlement),
    'Err' : IDL.Text,
  });
  const AgentBalance = IDL.Record({
    'principal' : IDL.Principal,
    'total_deposits' : IDL.Nat64,
    'total_commission_owed' : IDL.Nat64,
    'total_commission_paid' : IDL.Nat64,
    'last_settlement_date' : IDL.Opt(IDL.Nat64),
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    'confirm_deposit' : IDL.Func([ConfirmDepositRequest], [Result], []),
    'create_deposit_request' : IDL.Func([CreateDepositRequest], [Result], []),
    'create_monthly_settlement' : IDL.Func([IDL.Text], [Result_1], []),
    'get_agent_balance' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(AgentBalance)],
        ['query'],
      ),
    'get_agent_deposits' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(DepositTransaction)],
        ['query'],
      ),
    'get_agent_settlements' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(MonthlySettlement)],
        ['query'],
      ),
    'get_all_agent_balances' : IDL.Func([], [IDL.Vec(AgentBalance)], ['query']),
    'get_commission_rate' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_deposit' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(DepositTransaction)],
        ['query'],
      ),
    'get_pending_deposits' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(DepositTransaction)],
        ['query'],
      ),
    'get_settlements_for_month' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(MonthlySettlement)],
        ['query'],
      ),
    'get_total_revenue' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_user_deposits' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(DepositTransaction)],
        ['query'],
      ),
    'mark_settlement_paid' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_2],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
