rustup target add aarch64-apple-ios	
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
rustup target add arm-linux-androideabi
rustup target add armv7-linux-androideabi
cargo build --target=x86_64-apple-ios
cargo build --release 
