use crate::metadata::use_admin;
use crate::user::{BasicUserInfo, StakeItem, UserItem, Wallet};

use candid::{candid_method, Principal};
use ic_cdk::storage;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;

mod metadata;
mod types;
mod user;
mod utils;

thread_local! {
    static METADATA:RefCell<metadata::Metadata> = RefCell::default();
    static USER:RefCell<user::User> = RefCell::default();
}

#[init]
fn init() {
    METADATA.with(|metadata| {
        let mut meta = metadata.borrow_mut();
        meta.owner = Some(ic_cdk::caller());
        meta.canister_id = ic_cdk::api::id().to_text();
    });
}

#[update(guard = "use_admin")]
#[candid_method(update)]
pub fn add_admin(principal: Principal) -> Result<(), String> {
    METADATA.with(|data| data.borrow_mut().admin_list.push(principal));
    Ok(())
}

#[update]
#[candid_method(update)]
pub fn login(wallet_type: String) -> Result<user::UserItem, String> {
    USER.with(|data| data.borrow_mut().login(wallet_type))
}

#[update]
#[candid_method(update)]
pub fn bind_wallet(wallet: Wallet) -> Result<(), String> {
    USER.with(|data| data.borrow_mut().bind_wallet(wallet))
}

#[query]
#[candid_method(query)]
pub fn user_info() -> Result<user::UserItem, String> {
    USER.with(|data| data.borrow().user_info())
}
#[update]
#[candid_method(update)]
pub fn update_user_info(user: BasicUserInfo) -> Result<(), String> {
    USER.with(|data| data.borrow_mut().update_user_info(user))
}

#[update]
#[candid_method(update)]
pub fn add_stake(item: StakeItem) -> Result<UserItem, String> {
    USER.with(|data| data.borrow_mut().add_stake(item))
}

#[query]
#[candid_method(query)]
pub fn system_time() -> u64 {
    ic_cdk::api::time()
}

#[query(guard = "use_admin")]
#[candid_method(query)]
fn metadata() -> Result<metadata::Metadata, String> {
    Ok(METADATA.with(|metadata| metadata.borrow().clone()))
}

///
///
/// stable data
///
///
#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((
        METADATA.with(|metadata| metadata.borrow().clone()),
        USER.with(|user| user.borrow().clone()),
    ))
    .unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (old_metadata, old_user): (metadata::Metadata, user::User) =
        storage::stable_restore().unwrap();
    METADATA.with(|data| *data.borrow_mut() = old_metadata);
    USER.with(|data| *data.borrow_mut() = old_user);
}

// candid interface
candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
