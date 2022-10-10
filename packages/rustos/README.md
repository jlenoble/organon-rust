Upgrading nightly:

```bash
rustup update
cargo clean
rustup override set nightly
rustup component add rust-src
cargo install bootimage
rustup component add llvm-tools-preview
cargo test
```

Last working nightly 2022-10-01