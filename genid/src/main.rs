use std::{
  collections::{BTreeMap, HashMap, HashSet},
  fs::{File, read, read_to_string, write},
  io::Write,
  path::{Path, PathBuf},
};

use aok::{OK, Result, Void};
use intbin::to_bin;
use log::info;
use serde::{Deserialize, Serialize};
use tosql::{SQL_STRUCT_LI, SqlStruct};
pub use used;
use xkv::{
  R,
  fred::interfaces::{FunctionInterface, HashesInterface},
};

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
  pub id: u64,
  #[serde(with = "serde_bytes")]
  pub dump: Vec<u8>,
}

fn read_cache(cache_path: impl AsRef<Path>) -> Result<HashMap<String, Struct>> {
  let result = read(cache_path.as_ref());

  Ok(match result {
    Ok(bytes) => serde_yaml_bw::from_slice(&bytes)?,
    Err(e) => {
      if e.kind() == std::io::ErrorKind::NotFound {
        Default::default()
      } else {
        return Err(e.into());
      }
    }
  })
}

pub const R_SITE_STRUCT: &[u8] = b"siteStruct";

#[tokio::main]
async fn main() -> Void {
  xboot::init().await?;
  let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR")?.into();
  let cache_fp = manifest_dir.join("cache.yml");
  let mut cache = read_cache(&cache_fp)?;

  let mut changed = HashSet::new();
  // 用 BTreeMap 保证顺序一致性
  let mut path_struct: BTreeMap<&'static str, Vec<(u64, &'static str)>> = Default::default();
  let mut name_path = HashMap::new();

  let p = R.pipeline();
  for site_struct in SQL_STRUCT_LI {
    let SqlStruct { path, meta } = site_struct;
    if let Some(exist) = name_path.get(&meta.name) {
      eprint!("❌ {}({path}) already exists in {exist}", meta.name);
      std::process::exit(1);
    }
    name_path.insert(meta.name, path);
    let dump = meta.dump();
    let id = if let Some(pre) = cache.get(meta.name) {
      if pre.dump == dump {
        continue;
      }
      pre.id
    } else {
      R.fcall("zsetId", &["siteStructId"], [meta.name]).await?
    };
    let _: () = p.hset(R_SITE_STRUCT, (to_bin(id), dump.clone())).await?;
    cache.insert(meta.name.into(), Struct { id, dump });
    info!("changed {path} {}", meta.name);
    changed.insert(path);
    path_struct.entry(path).or_default().push((id, meta.name));
  }
  if changed.is_empty() {
    return OK;
  }
  let _: () = p.last().await?;
  let root = manifest_dir.parent().unwrap();
  for path_str in changed {
    let mut li = Vec::new();
    let path: PathBuf = path_str.into();
    let out = root.join(path.parent().unwrap());
    if let Some(id_name_li) = path_struct.get(path_str) {
      for (id, name) in id_name_li {
        li.push(format!(
          r#"
#[distributed_slice]
pub static {name}: HookLi<crate::{name}>;

impl TypeHook for crate::{name} {{
  const ID: u64 = {id};
  const HOOK: HookSlice<Self> = &{name};
}}"#
        ));
      }
      {
        let mut file = File::create(out.join("site_log.rs"))?;
        file.write_all(
          r#"#![allow(non_upper_case_globals)]

use linkme::distributed_slice;
use site_log::{HookLi, HookSlice, TypeHook};
"#
          .as_bytes(),
        )?;
        file.write_all(li.join("\n").as_bytes())?;
        file.write_all(b"\n")?;
      }
      {
        let lib_rs_path = out.join("lib.rs");
        let lib_rs = read_to_string(&lib_rs_path)?;
        let lib_rs = lib_rs.trim();

        if !lib_rs.contains("\nmod site_log;") {
          let mut file = File::create(lib_rs_path)?;
          let mut iter = lib_rs.lines();
          for i in iter.by_ref() {
            if i.starts_with("#") {
              file.write_all(i.as_bytes())?;
              file.write_all(b"\n")?;
            } else {
              break;
            }
          }
          file.write_all(
            br#"
mod site_log;
pub use site_log::*;
"#,
          )?;
          for i in iter {
            file.write_all(i.as_bytes())?;
            file.write_all(b"\n")?;
          }
        }
      }
    }
  }
  write(cache_fp, serde_yaml_bw::to_string(&cache)?)?;
  OK
}
