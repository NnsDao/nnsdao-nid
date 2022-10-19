use candid::{CandidType, Deserialize, Principal};

use crate::METADATA;

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct Metadata {
    pub owner: Option<Principal>,
    pub admin_list: Vec<Principal>,
    pub canister_id: String,
}

pub fn is_owner(from: Principal) -> bool {
    METADATA.with(|metadata| metadata.borrow().owner == Some(from))
}
pub fn is_admin(from: Principal) -> bool {
    METADATA.with(|metadata| metadata.borrow().admin_list.contains(&from))
}
pub fn use_admin() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if is_admin(caller) | is_owner(caller) {
        Ok(())
    } else {
        Err("Not admin".to_string())
    }
}
