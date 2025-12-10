#![allow(non_upper_case_globals)]

use linkme::distributed_slice;
use site_log::{HookLi, HookSlice, TypeHook};

#[distributed_slice]
pub static Host: HookLi<crate::Host>;

impl TypeHook for crate::Host {
  const ID: u64 = 40;
  const HOOK: HookSlice<Self> = &Host;
}

#[distributed_slice]
pub static SiteHost: HookLi<crate::SiteHost>;

impl TypeHook for crate::SiteHost {
  const ID: u64 = 41;
  const HOOK: HookSlice<Self> = &SiteHost;
}

#[distributed_slice]
pub static SiteAdmin: HookLi<crate::SiteAdmin>;

impl TypeHook for crate::SiteAdmin {
  const ID: u64 = 42;
  const HOOK: HookSlice<Self> = &SiteAdmin;
}

#[distributed_slice]
pub static SiteOwner: HookLi<crate::SiteOwner>;

impl TypeHook for crate::SiteOwner {
  const ID: u64 = 43;
  const HOOK: HookSlice<Self> = &SiteOwner;
}
