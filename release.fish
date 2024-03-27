#!/usr/bin/fish
cargo build --release
scp target/release/radiocrouton p6nj@85.215.159.253:~/