#!/usr/bin/fish
cargo build
and ./kill.fish
and rsync -u --progress target/debug/radiocrouton p6nj@85.215.159.253:~/server
and ssh p6nj@85.215.159.253 "sudo ./server"