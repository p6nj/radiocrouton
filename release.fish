#!/usr/bin/fish
cargo build --release
and ./kill.fish
and rsync -u --progress target/release/radiocrouton p6nj@85.215.159.253:~/server
and ssh p6nj@85.215.159.253 "sudo ./server"