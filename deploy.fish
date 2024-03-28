#!/usr/bin/fish
cargo build --release
and ssh p6nj@85.215.159.253 "sudo systemctl stop radiocrouton"
and rsync -u --progress target/release/radiocrouton p6nj@85.215.159.253:~/server
and ssh p6nj@85.215.159.253 "sudo systemctl start radiocrouton"