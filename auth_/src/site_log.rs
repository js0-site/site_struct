#![allow(non_upper_case_globals)]

use linkme::distributed_slice;
use site_log::{HookLi, HookSlice, TypeHook};

#[distributed_slice]
pub static Mail: HookLi<crate::Mail>;

impl TypeHook for crate::Mail {
  const ID: u64 = 33;
  const HOOK: HookSlice<Self> = &Mail;
}

#[distributed_slice]
pub static Name: HookLi<crate::Name>;

impl TypeHook for crate::Name {
  const ID: u64 = 34;
  const HOOK: HookSlice<Self> = &Name;
}

#[distributed_slice]
pub static User: HookLi<crate::User>;

impl TypeHook for crate::User {
  const ID: u64 = 35;
  const HOOK: HookSlice<Self> = &User;
}

#[distributed_slice]
pub static UserMail: HookLi<crate::UserMail>;

impl TypeHook for crate::UserMail {
  const ID: u64 = 36;
  const HOOK: HookSlice<Self> = &UserMail;
}

#[distributed_slice]
pub static UserName: HookLi<crate::UserName>;

impl TypeHook for crate::UserName {
  const ID: u64 = 37;
  const HOOK: HookSlice<Self> = &UserName;
}

#[distributed_slice]
pub static UserPassword: HookLi<crate::UserPassword>;

impl TypeHook for crate::UserPassword {
  const ID: u64 = 38;
  const HOOK: HookSlice<Self> = &UserPassword;
}

#[distributed_slice]
pub static UserMailPrimary: HookLi<crate::UserMailPrimary>;

impl TypeHook for crate::UserMailPrimary {
  const ID: u64 = 39;
  const HOOK: HookSlice<Self> = &UserMailPrimary;
}
