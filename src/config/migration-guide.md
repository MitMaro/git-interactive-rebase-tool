# Migration Guide

## 1.x.x to 2.0.0
- `Config::new()` no longer automatically loads configuration from the repository. Instead, use `Config::try_from` providing a `git::Repository` instance.

```rust
use git::Repository;
use config::Config;

fn main() {
	let config = Config::try_from(&Repository::open_from_env().unwrap());
}
```

* `Config::new()` now returns a default instance that is almost identical to the `create_config` test utility, and is a drop in replacement in most cases.

* `Theme::new()` returns a default instance that is almost identical to the `create_theme` test utility, and is a drop in replacement in most cases.