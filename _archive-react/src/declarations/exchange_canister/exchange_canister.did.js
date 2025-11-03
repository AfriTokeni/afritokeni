export const idlFactory = ({ IDL }) => {
  const Token = IDL.Variant({ 'CkUSDC' : IDL.Null, 'CkBTC' : IDL.Null });
  const ExchangeRequest = IDL.Record({
    'to_token' : Token,
    'from_token' : Token,
    'min_output' : IDL.Nat64,
    'amount' : IDL.Nat64,
  });
  const ExchangeResult = IDL.Record({
    'tx_id' : IDL.Text,
    'output_amount' : IDL.Nat64,
    'spread_amount' : IDL.Nat64,
  });
  const Result = IDL.Variant({ 'Ok' : ExchangeResult, 'Err' : IDL.Text });
  return IDL.Service({
    'get_company_wallet' : IDL.Func([], [IDL.Text], ['query']),
    'get_dex_provider' : IDL.Func([], [IDL.Text], ['query']),
    'get_sonic_canister' : IDL.Func([], [IDL.Text], ['query']),
    'get_spread_percentage' : IDL.Func([], [IDL.Nat64], ['query']),
    'swap_tokens' : IDL.Func([ExchangeRequest], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
