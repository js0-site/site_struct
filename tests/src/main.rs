use std::sync::LazyLock;

use anyhow::Result;
use argon2::{Algorithm, Argon2, Params, Version};
use subtle::ConstantTimeEq; // 引入常量时间比较 trait

static ARGON2_PARAMS: LazyLock<Params> = LazyLock::new(|| {
  Params::new(
    65536, // m_cost (内存)
    3,     // t_cost (时间/迭代)
    4,     // p_cost (并行度)
    None,  // output len
  )
  .unwrap()
});

static ARGON2: LazyLock<Argon2> =
  LazyLock::new(|| Argon2::new(Algorithm::Argon2id, Version::V0x13, ARGON2_PARAMS.clone()));

fn main() -> Result<()> {
  // ==========================================
  // 1. 模拟注册（你给出的代码）
  // ==========================================
  let password = b"password123"; // 用户注册时的密码
  let salt = b"0123456789abcdef"; // 必须是随机生成的 16 字节盐 (存入 DB)

  let mut stored_hash = [0u8; 32]; // 最终生成的 Hash (存入 DB)

  // 注意：这里使用了 default()，意味着使用了默认的 m_cost, t_cost 等参数
  ARGON2
    .hash_password_into(password, salt, &mut stored_hash)
    .map_err(|e| anyhow::anyhow!(e))?;

  println!("--- 数据库状态 ---");
  println!("Stored Salt: {:?}", salt);
  println!("Stored Hash: {}", hex::encode(stored_hash));

  // ==========================================
  // 2. 模拟登录（验证逻辑）
  // ==========================================

  let input_password_correct = b"password123"; // 用户输入的正确密码
  let input_password_wrong = b"password000"; // 用户输入的错误密码

  println!("\n--- 开始验证 ---");

  // 验证测试 1：正确的密码
  let is_valid = verify_login(input_password_correct, salt, &stored_hash)?;
  println!(
    "输入 'password123' 验证结果: {}",
    if is_valid { "✅ 通过" } else { "❌ 失败" }
  );

  // 验证测试 2：错误的密码
  let is_valid = verify_login(input_password_wrong, salt, &stored_hash)?;
  println!(
    "输入 'password000' 验证结果: {}",
    if is_valid { "✅ 通过" } else { "❌ 失败" }
  );

  Ok(())
}

/// 验证函数
/// input_pw: 用户登录输入的密码
/// db_salt:  从数据库取出的盐
/// db_hash:  从数据库取出的 Hash
fn verify_login(input_pw: &[u8], db_salt: &[u8], db_hash: &[u8]) -> Result<bool> {
  // 1. 准备一个容器来存放“重新计算”的 Hash
  let mut check_hash = [0u8; 32];

  // 2. 关键点：必须使用和注册时完全一样的参数配置！
  // 因为注册时用了 Argon2::default()，这里验证也必须用 Argon2::default()
  Argon2::hash_password_into(&ARGON2, input_pw, db_salt, &mut check_hash)
    .map_err(|e| anyhow::anyhow!(e))?;

  // 3. 比较计算结果和数据库存的 Hash
  // ⚠️ 安全警告：不要使用 `check_hash == db_hash`
  // 普通的 == 比较会在发现第一个字节不同时立即返回，这会让黑客通过响应时间猜出 Hash。
  // 使用 ct_eq (Constant Time Eq) 保证无论是否相等，比较耗时都一样。
  let is_match = check_hash.ct_eq(db_hash).into();

  Ok(is_match)
}
