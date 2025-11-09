// DAO governance functions
use super::{get_business_logic_canister_id, DaoProposal};
use ic_cdk::call::Call;

/// Get DAO proposals
pub async fn get_dao_proposals() -> Result<Vec<DaoProposal>, String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling get_dao_proposals");
    
    let response = Call::unbounded_wait(canister_id, "get_dao_proposals")
        .with_args(&())
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<Vec<DaoProposal>, String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}

/// Cast DAO vote
pub async fn cast_dao_vote(phone_number: &str, proposal_id: u64, vote_yes: bool, pin: &str) -> Result<(), String> {
    let canister_id = get_business_logic_canister_id()?;
    
    ic_cdk::println!("📤 Calling cast_dao_vote: phone={}, proposal={}, vote={}", phone_number, proposal_id, vote_yes);
    
    let response = Call::unbounded_wait(canister_id, "cast_dao_vote")
        .with_args(&(phone_number, proposal_id, vote_yes, pin))
        .await
        .map_err(|e| format!("Call failed: {:?}", e))?;
    
    let (result,): (Result<(), String>,) = response
        .candid_tuple()
        .map_err(|e| format!("Decode failed: {}", e))?;
    
    result
}
