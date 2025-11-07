// Stable storage using ic-stable-structures
// This persists data across canister calls AND upgrades

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StableData {
    pub pins: HashMap<String, String>,
    pub balances: HashMap<String, crate::utils::datastore::Balance>,
    pub languages: HashMap<String, String>,
    pub data: HashMap<String, HashMap<String, String>>,
    pub pin_attempts: HashMap<String, u32>,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
        
    static STABLE_STORE: RefCell<StableCell<Vec<u8>, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            Vec::new()
        ).expect("Failed to initialize stable store")
    );
}

/// Get stable data
pub fn get_stable_data() -> StableData {
    STABLE_STORE.with(|store| {
        let bytes = store.borrow().get().clone();
        if bytes.is_empty() {
            StableData::default()
        } else {
            serde_json::from_slice(&bytes).unwrap_or_default()
        }
    })
}

/// Set stable data
pub fn set_stable_data(data: &StableData) {
    STABLE_STORE.with(|store| {
        let bytes = serde_json::to_vec(data).expect("Failed to serialize stable data");
        store.borrow_mut().set(bytes).expect("Failed to save stable data");
    });
}

/// Get user language from stable storage
pub fn get_user_language_stable(phone: &str) -> Option<String> {
    let data = get_stable_data();
    data.languages.get(phone).cloned()
}

/// Set user language in stable storage
pub fn set_user_language_stable(phone: &str, lang: &str) {
    let mut data = get_stable_data();
    data.languages.insert(phone.to_string(), lang.to_string());
    set_stable_data(&data);
}

/// Get PIN attempts from stable storage
pub fn get_pin_attempts_stable(phone: &str) -> u32 {
    let data = get_stable_data();
    data.pin_attempts.get(phone).copied().unwrap_or(0)
}

/// Increment PIN attempts in stable storage
pub fn increment_pin_attempts_stable(phone: &str) -> u32 {
    let mut data = get_stable_data();
    let count = data.pin_attempts.get(phone).copied().unwrap_or(0) + 1;
    data.pin_attempts.insert(phone.to_string(), count);
    set_stable_data(&data);
    count
}

/// Reset PIN attempts in stable storage
pub fn reset_pin_attempts_stable(phone: &str) {
    let mut data = get_stable_data();
    data.pin_attempts.remove(phone);
    set_stable_data(&data);
}
