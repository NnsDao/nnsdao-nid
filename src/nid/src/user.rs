use crate::metadata::is_admin;
use crate::utils::{find_binding_nid, get_nid, NIDType, PrincipalIdText};
use candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub(crate) struct User {
    pub(crate) member: HashMap<NIDType, UserItem>,
    pub(crate) binding_wallet: Vec<Wallet>,
}

impl User {
    pub(crate) fn login(&mut self, wallet_type: String) -> Result<TotalUserInfo, String> {
        let caller = ic_cdk::caller();
        if let Err(_str) = find_binding_nid(self) {
            let nid = get_nid();
            self.binding_wallet
                .push(Wallet(nid.clone(), wallet_type, caller.to_text()));
            self.member.insert(
                nid.clone(),
                UserItem {
                    level: 1,
                    nid,
                    nickname: caller.to_text(),
                    last_login_at: ic_cdk::api::time(),
                    ..Default::default()
                },
            );
        };
        self.user_info()
    }
    pub(crate) fn user_info(&self) -> Result<TotalUserInfo, String> {
        // let caller = ic_cdk::caller();
        let nid = find_binding_nid(self)?;
        // let caller = ic_cdk::caller();
        let item = self.member.get(&nid).unwrap().clone();
        let UserItem {
            nickname,
            avatar,
            intro,
            nid,
            credit,
            level,
            log,
            badge,
            stake,
            last_login_at,
            social,
        } = item;

        let wallet: Vec<Wallet> = self
            .binding_wallet
            .clone()
            .into_iter()
            .filter(|wallet| wallet.0 == nid)
            .collect();

        Ok(TotalUserInfo {
            nickname,
            avatar,
            intro,
            nid,
            credit,
            level,
            log,
            badge,
            stake,
            wallet,
            last_login_at,
            social,
        })
    }
    pub(crate) fn update_user_info(
        &mut self,
        user: BasicUserInfo,
    ) -> Result<TotalUserInfo, String> {
        let nid = find_binding_nid(self)?;
        // if nid != user.nid {
        //     return Err("NID does not match".to_string());
        // }
        let data = self.member.get(&nid).unwrap();
        let new_data = UserItem {
            nickname: user.nickname,
            avatar: user.avatar,
            intro: user.intro,
            social: user.social,
            ..data.clone()
        };
        self.member.insert(nid, new_data);
        self.user_info()
    }
    pub(crate) fn bind_wallet(&mut self, wallet: Wallet) -> Result<(), String> {
        let nid = find_binding_nid(self)?;
        if nid == wallet.0 {
            self.is_already_bind(nid, wallet.2.clone())?;
            self.binding_wallet.push(wallet);
            Ok(())
        } else {
            Err("Invalid NID".to_string())
        }
    }
    fn is_already_bind(&self, nid: NIDType, principal_id_text: String) -> Result<(), String> {
        for wallet in &self.binding_wallet {
            if wallet.0 == nid && wallet.2 == principal_id_text {
                return Err("Current wallet has already bind".to_string());
            }
        }
        Ok(())
    }

    pub(crate) fn add_stake(&mut self, item: StakeItem) -> Result<UserItem, String> {
        let from = ic_cdk::caller();
        if !is_admin(from) {
            return Err("Unauthorized operation!".to_string());
        }
        let nid = find_binding_nid(self)?;
        let data = self.member.get_mut(&nid).unwrap();
        data.stake.push(StakeItem { ..item });
        Ok(data.clone())
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
    detail: String,
    time: i64,
}
#[derive(Deserialize, CandidType, Debug, Clone)]
pub struct StakeItem {
    project: String,
    profit: String,
    start_time: u64,
    duration: StakeItemDuration,
    status: StakeItemStatus,
}
#[derive(Deserialize, CandidType, Debug, Default, Clone)]
pub struct TotalUserInfo {
    nickname: String,
    avatar: String,
    intro: String,
    nid: NIDType,
    credit: u64,
    social: HashMap<String, String>,
    level: u64,
    log: Vec<UserLog>,
    // asset: HashMap<String, AssertTokenItem>,
    badge: Vec<String>,
    stake: Vec<StakeItem>,
    wallet: Vec<Wallet>,
    last_login_at: u64,
}

#[derive(Deserialize, CandidType, Debug, Default, Clone)]
pub struct UserItem {
    nickname: String,
    avatar: String,
    intro: String,
    nid: NIDType,
    credit: u64,
    level: u64,
    social: HashMap<String, String>,
    log: Vec<UserLog>,
    // asset: HashMap<String, AssertTokenItem>,
    badge: Vec<String>,
    stake: Vec<StakeItem>,
    last_login_at: u64,
}

#[derive(Deserialize, CandidType, Debug, Clone)]
pub struct BasicUserInfo {
    nickname: String,
    avatar: String,
    intro: String,
    social: HashMap<String, String>,
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
