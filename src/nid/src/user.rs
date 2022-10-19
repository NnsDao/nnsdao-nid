use crate::metadata::is_owner;
use crate::utils::{find_binding_nid, get_nid, NIDType, PrincipalIdText};
use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub(crate) struct User {
    pub(crate) member: HashMap<NIDType, UserItem>,
    pub(crate) binding_wallet: Vec<Wallet>,
}

impl User {
    pub(crate) fn login(&mut self, wallet_type: String) -> Result<UserItem, String> {
        let caller = ic_cdk::caller();
        match find_binding_nid(self) {
            Ok(nid) => return Ok(self.member.get(&nid).unwrap().clone()),
            Err(_str) => {
                let nid = get_nid();
                self.binding_wallet
                    .push(Wallet(nid, wallet_type, caller.to_text()));
                self.member.insert(
                    nid,
                    UserItem {
                        level: 1,
                        nid,
                        ..Default::default()
                    },
                );
                return Ok(self.member.get(&nid).unwrap().clone());
            }
        }
    }
    pub(crate) fn user_info(&self) -> Result<UserItem, String> {
        // let caller = ic_cdk::caller();
        let nid = find_binding_nid(self)?;
        Ok(self.member.get(&nid).unwrap().clone())
    }
    pub(crate) fn update_user_info(&mut self, user: BasicUserInfo) -> Result<(), String> {
        let nid = find_binding_nid(self)?;
        if nid != user.nid {
            return Err("NID does not match".to_string());
        }
        let data = self.member.get(&nid).unwrap();
        self.member.insert(
            nid,
            UserItem {
                nickname: user.nickname,
                avatar: user.avatar,
                intro: user.intro,
                ..data.clone()
            },
        );
        Ok(())
    }
    pub(crate) fn bind_wallet(&mut self, wallet: Wallet) -> Result<(), String> {
        let nid = find_binding_nid(self)?;
        if nid == wallet.0 {
            self.binding_wallet.push(wallet);
            Ok(())
        } else {
            Err("Invalid NID".to_string())
        }
    }
    pub(crate) fn add_stake() -> Result<(), String> {
        let from = ic_cdk::caller();
        if !is_owner(from) {
            return Err("Unauthorized operation!".to_string());
        }
        todo!();
        Ok(())
    }
}

// sub struct

#[derive(Deserialize, CandidType, Debug, Clone)]
pub struct Wallet(pub NIDType, pub String, pub PrincipalIdText); // ("plug","principalIdText")

#[derive(Deserialize, CandidType, Debug, Clone)]
struct AssertTokenItem {
    profit: i64,
    total: i64,
    withdraw: i64,
}

#[derive(Deserialize, CandidType, Debug, Clone)]
struct UserLog {
    event: String,
    time: i64,
}
#[derive(Deserialize, CandidType, Debug, Clone)]
struct StakeItem {
    project: String,
    interest: String,
    start_time: u64,
    duration: StakeItemDuration,
    status: StakeItemStatus,
}
#[derive(Deserialize, CandidType, Debug, Default, Clone)]
pub struct UserItem {
    nickname: String,
    avatar: String,
    intro: String,
    nid: NIDType,
    asset: HashMap<String, AssertTokenItem>,
    badge: Vec<String>,
    credit: u64,
    level: u64,
    log: Vec<UserLog>,
    stake: Vec<StakeItem>,
}

#[derive(Deserialize, CandidType, Debug, Clone)]
pub struct BasicUserInfo {
    nid: NIDType,
    nickname: String,
    avatar: String,
    intro: String,
}

#[derive(Deserialize, CandidType, Debug, Clone)]
pub(crate) enum StakeItemStatus {
    Valid,
    Expired,
    Banned,
}

impl Default for StakeItemStatus {
    fn default() -> Self {
        StakeItemStatus::Valid
    }
}

#[derive(Deserialize, CandidType, Debug, Clone)]
pub(crate) enum StakeItemDuration {
    LongTerm,
    Month(u32),
}

impl Default for StakeItemDuration {
    fn default() -> Self {
        StakeItemDuration::LongTerm
    }
}
