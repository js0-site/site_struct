#![allow(non_upper_case_globals)]

use linkme::distributed_slice;
use site_log::{HookLi, HookSlice, TypeHook};

#[distributed_slice]
pub static Domain: HookLi<crate::Domain>;

impl TypeHook for crate::Domain {
  const ID: u64 = 33;
  const HOOK: HookSlice<Self> = &Domain;
}

#[distributed_slice]
pub static SiteAdmin: HookLi<crate::SiteAdmin>;

impl TypeHook for crate::SiteAdmin {
  const ID: u64 = 34;
  const HOOK: HookSlice<Self> = &SiteAdmin;
}

#[distributed_slice]
pub static SiteOwner: HookLi<crate::SiteOwner>;

impl TypeHook for crate::SiteOwner {
  const ID: u64 = 35;
  const HOOK: HookSlice<Self> = &SiteOwner;
}
