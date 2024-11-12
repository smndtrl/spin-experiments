curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-binstall
cargo binstall cargo-component

sudo apt update
sudo apt install pkg-config libsqlite3-dev