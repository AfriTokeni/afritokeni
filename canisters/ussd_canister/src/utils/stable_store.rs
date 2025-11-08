// Simplified stable storage using thread_local with manual persistence
// For true persistence, data should be in Juno satellite
// This is a temporary solution until USSD is merged into satellite

use std::cell::RefCell;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StableData {
    pub languages: HashMap<String, String>,
    pub pin_attempts: HashMap<String, u32>,
}

thread_local! {
    static STABLE_DATA: RefCell<StableData> = RefCell::new(StableData::default());
}

/// Get stable data
pub fn get_stable_data() -> StableData {
    STABLE_DATA.with(|data| data.borrow().clone())
}

/// Set stable data
pub fn set_stable_data(new_data: &StableData) {
    STABLE_DATA.with(|data| {
        *data.borrow_mut() = new_data.clone();
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
