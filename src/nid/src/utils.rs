use crate::user::{User, Wallet};

pub static mut NID: u64 = 0;
pub type NIDType = u64;
pub(crate) type PrincipalIdText = String;

pub(crate) fn get_nid() -> u64 {
    unsafe {
        NID += 1;
        NID
    }
}
pub(crate) fn find_binding_nid(user: &User) -> Result<u64, String> {
    let caller = ic_cdk::caller();
    for Wallet(_nid, _name, principal) in &user.binding_wallet {
        if *principal == caller.to_text() {
            return Ok(*_nid);
        }
    }
    Err("Can not find binding NID,please bind NID first at nomos platform".to_owned())
}
pub(crate) fn is_own(user: &User, nid: NIDType) -> bool {
    let caller = ic_cdk::caller();

    for Wallet(_nid, _name, principal) in &user.binding_wallet {
        if *principal == caller.to_text() {
            return *_nid == nid;
        }
    }
    false
}
