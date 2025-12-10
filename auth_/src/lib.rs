#![cfg_attr(docsrs, feature(doc_cfg))]

mod site_log;
pub use site_log::*;
use tosql::tosql;

#[tosql]
pub struct User {
  pub id: u64,
}

#[tosql]
pub struct Name {
  pub id: u64,
  pub val: String,
}

#[tosql]
pub struct UserName {
  pub id: u64,
  pub name_id: u64,
}

#[tosql]
pub struct Mail {
  pub id: u64,
  pub val: String,
  pub host_id: u64,
}

#[tosql]
pub struct UserMailPrimary {
  pub id: u64,
  pub user_id: u64,
  pub mail_id: u64,
}

#[tosql]
pub struct UserMail {
  pub id: u64,
  pub user_id: u64,
  pub mail_id: u64,
  pub state: i8, // -1 rm ; 0 hidden; 1 active
}

/*
use argon2::Argon2;

let password = b"password"; // Bad password; don't actually use!
let salt = b"example salt"; // Salt should be unique per password
let mut hash = [0u8; 32]; // Can be any desired size
Argon2::default().hash_password_into(password, salt, &mut hash)?;
*/

#[tosql]
pub struct UserPassword {
  pub id: u64,
  pub v: u8,
  pub hash: [u8; 32],
  pub salt: [u8; 16],
}
