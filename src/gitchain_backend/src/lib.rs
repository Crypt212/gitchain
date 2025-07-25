// src/lib.rs
use ic_cdk::{update, query};
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, candid::CandidType)]
struct Repo {
    name: String,
    cid: String,
    owner: String,
    timestamp: u64,
}

thread_local! {
    static REPOS: RefCell<HashMap<String, Repo>> = RefCell::new(HashMap::new());
}

#[update]
fn store_repo(name: String, cid: String) -> String {
    let owner = ic_cdk::caller().to_string();
    let timestamp = ic_cdk::api::time();
    
    let new_repo = Repo {
        name,
        cid: cid.clone(),
        owner,
        timestamp,
    };
    
    REPOS.with(|repos| {
        repos.borrow_mut().insert(cid.clone(), new_repo);
    });
    
    format!("Repo stored with CID: {}", cid)
}

#[query]
fn get_repo(cid: String) -> Option<Repo> {
    REPOS.with(|repos| repos.borrow().get(&cid).cloned())
}

#[query]
fn get_user_repos(owner: String) -> Vec<Repo> {
    REPOS.with(|repos| {
        repos.borrow()
            .values()
            .filter(|repo| repo.owner == owner)
            .cloned()
            .collect()
    })
}

ic_cdk::export_candid!();
