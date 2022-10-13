use crate::metadata::use_admin;
use candid::candid_method;
use ic_cdk::storage;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query};
use std::cell::RefCell;

mod metadata;
mod types;
mod utils;

thread_local! {
    static METADATA:RefCell<metadata::Metadata> = RefCell::default();
}

#[init]
fn init() {
    METADATA.with(|metadata| {
        let mut meta = metadata.borrow_mut();
        meta.owner = Some(ic_cdk::caller());
        meta.canister_id = ic_cdk::api::id().to_text();
    });
}

#[query(guard = "use_admin")]
#[candid_method(query)]
fn metadata() -> Result<metadata::Metadata, String> {
    Ok(METADATA.with(|metadata| metadata.borrow().clone()))
}

#[query]
#[candid_method(query)]
pub fn system_time() -> u64 {
    ic_cdk::api::time()
}

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((METADATA.with(|metadata| metadata.borrow().clone()),)).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let (old_metadata,): (metadata::Metadata,) = storage::stable_restore().unwrap();
    METADATA.with(|data| *data.borrow_mut() = old_metadata);
}

// candid interface
candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
