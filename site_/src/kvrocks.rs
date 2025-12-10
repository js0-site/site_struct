// use site_log::linkme;
// use xkv::{R, fred::interfaces::SortedSetsInterface};
//
// pub const R_SITE_DOMAIN: &[u8] = b"siteDomain";
//
// linkme!(|domain| {
//   let domain_name = domain.name.as_bytes();
//   let _: () = if domain.state == 1 {
//     R.zadd(
//       R_SITE_DOMAIN,
//       None,
//       None,
//       false,
//       false,
//       (domain.ctx.site_id as f64, domain_name),
//     )
//     .await?
//   } else {
//     R.zrem(R_SITE_DOMAIN, domain_name).await?
//   };
// });
