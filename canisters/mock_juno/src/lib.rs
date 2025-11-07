use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{query, update};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SetDoc {
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub version: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Doc {
    pub updated_at: u64,
    pub owner: Principal,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub created_at: u64,
    pub version: Option<u64>,
}

thread_local! {
    static STORE: RefCell<HashMap<String, HashMap<String, Doc>>> = RefCell::new(HashMap::new());
}

fn make_key(collection: &str, key: &str) -> String {
    format!("{}:{}", collection, key)
}

#[query]
fn get_doc(collection: String, key: String) -> Option<Doc> {
    let full_key = make_key(&collection, &key);
    STORE.with(|store| {
        store.borrow()
            .get(&collection)
            .and_then(|coll| coll.get(&key).cloned())
    })
}

#[update]
fn set_doc(collection: String, key: String, doc: SetDoc) -> Doc {
    let now = ic_cdk::api::time();
    let caller = ic_cdk::caller();
    
    let new_doc = Doc {
        updated_at: now,
        owner: caller,
        data: doc.data,
        description: doc.description,
        created_at: now,
        version: doc.version,
    };
    
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        store.entry(collection.clone())
            .or_insert_with(HashMap::new)
            .insert(key.clone(), new_doc.clone());
    });
    
    ic_cdk::println!("📝 Mock Juno: set_doc({}, {}) - {} bytes", collection, key, new_doc.data.len());
    
    new_doc
}

#[update]
fn delete_doc(collection: String, key: String) -> Option<Doc> {
    STORE.with(|store| {
        let mut store = store.borrow_mut();
        store.get_mut(&collection)
            .and_then(|coll| coll.remove(&key))
    })
}

// Export candid
ic_cdk::export_candid!();
