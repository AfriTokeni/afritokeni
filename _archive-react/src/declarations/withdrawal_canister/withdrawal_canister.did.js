export const idlFactory = ({ IDL }) => {
  const ConfirmWithdrawalRequest = IDL.Record({
    'agent_principal' : IDL.Principal,
    'withdrawal_code' : IDL.Text,
  });
  const TransactionStatus = IDL.Variant({
    'Confirmed' : IDL.Null,
    'Cancelled' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const WithdrawalTransaction = IDL.Record({
    'id' : IDL.Nat64,
    'status' : TransactionStatus,
    'user_principal' : IDL.Principal,
    'agent_principal' : IDL.Principal,
    'platform_fee_ugx' : IDL.Nat64,
    'amount_ugx' : IDL.Nat64,
    'agent_fee_ugx' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
    'withdrawal_code' : IDL.Text,
  });
  const Result = IDL.Variant({
    'Ok' : WithdrawalTransaction,
    'Err' : IDL.Text,
  });
  const CreateWithdrawalRequest = IDL.Record({
    'user_principal' : IDL.Principal,
    'agent_principal' : IDL.Principal,
    'amount_ugx' : IDL.Nat64,
  });
  const AgentEarnings = IDL.Record({
    'principal' : IDL.Principal,
    'total_withdrawals_processed' : IDL.Nat64,
    'last_withdrawal_date' : IDL.Opt(IDL.Nat64),
    'total_fees_earned' : IDL.Nat64,
    'total_fees_withdrawn' : IDL.Nat64,
  });
  return IDL.Service({
    'confirm_withdrawal' : IDL.Func([ConfirmWithdrawalRequest], [Result], []),
    'create_withdrawal_request' : IDL.Func(
        [CreateWithdrawalRequest],
        [Result],
        [],
      ),
    'get_agent_earnings' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(AgentEarnings)],
        ['query'],
      ),
    'get_agent_withdrawals' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(WithdrawalTransaction)],
        ['query'],
      ),
    'get_all_agent_earnings' : IDL.Func(
        [],
        [IDL.Vec(AgentEarnings)],
        ['query'],
      ),
    'get_fee_split' : IDL.Func([], [IDL.Nat64, IDL.Nat64], ['query']),
    'get_pending_withdrawals' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(WithdrawalTransaction)],
        ['query'],
      ),
    'get_total_agent_earnings' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_total_platform_revenue' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_user_withdrawals' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(WithdrawalTransaction)],
        ['query'],
      ),
    'get_withdrawal' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(WithdrawalTransaction)],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
