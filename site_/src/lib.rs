#![cfg_attr(docsrs, feature(doc_cfg))]

mod site_log;
pub use site_log::*;
mod kvrocks;
use tosql::tosql;

#[tosql]
pub struct Host {
  pub id: u64,
  pub val: String,
}

#[tosql]
pub struct SiteHost {
  pub id: u64,
  pub host_id: u64,
  pub state: i8, // 0: rm; 1: add
}

#[tosql]
pub struct SiteAdmin {
  pub id: u64,
  pub user_id: u64,
  pub state: i8, // 0: rm; 1: add
}

#[tosql]
pub struct SiteOwner {
  pub id: u64,
  pub user_id: u64,
}
