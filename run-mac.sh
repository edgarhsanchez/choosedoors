cargo build --target x86_64-apple-darwin --release
cargo build --target aarch64-apple-darwin --release
lipo -create target/x86_64-apple-darwin/release/choosedoors target/aarch64-apple-darwin/release/choosedoors -output target/release/choosedoors
./target/release/choosedoors
```