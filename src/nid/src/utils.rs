use crate::user::{User, Wallet};
use uuid::Uuid;

pub type NIDType = String;
pub(crate) type PrincipalIdText = String;

pub(crate) fn get_nid() -> String {
    Uuid::new_v4().to_string()
}
pub(crate) fn find_binding_nid(user: &User) -> Result<String, String> {
    let caller = ic_cdk::caller();
    for Wallet(_nid, _name, principal) in &user.binding_wallet {
        if *principal == caller.to_text() {
            return Ok(_nid.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        println!("new uuid {}", get_nid())
    }
}
