#!/usr/bin/fish
cargo build
scp target/debug/radiocrouton p6nj@85.215.159.253:~/
ssh p6nj@85.215.159.253 ./radiocrouton