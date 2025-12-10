用到的目录在 `./used/src/lib.rs` 中定义

运行 `./genid/run.sh` 生成类型的 `id`

会生成类似 `./site_/src/site_log.rs`

回调的写法类似下面

```
use site_log::linkme;

linkme!(|host| {
  let host = host.val.as_bytes();
  ...
});
```