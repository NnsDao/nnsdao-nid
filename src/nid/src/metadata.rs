use candid::{CandidType, Deserialize, Principal};

use crate::METADATA;

#[derive(Default, CandidType, Deserialize, Clone)]
pub(crate) struct Metadata {
    pub(crate) owner: Option<Principal>,
    pub(crate) admin_list: Vec<Principal>,
    pub(crate) canister_id: String,
}

pub fn is_owner() -> bool {
    METADATA.with(|metadata| metadata.borrow().owner == Some(ic_cdk::caller()))
}
pub fn is_admin() -> bool {
    METADATA.with(|metadata| metadata.borrow().admin_list.contains(&ic_cdk::caller()))
}
pub fn use_admin() -> Result<(), String> {
    if is_admin() | is_owner() {
        Ok(())
    } else {
        Err("Not admin".to_string())
    }
}
