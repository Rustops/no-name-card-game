#!/bin/bash

cargo build --release

cp ./target/release/server ./server/

if [ -z "$1" ] ;then
    echo "Use the default server port."
    echo "Server: [127.0.0.1:6666]"
    ./server/server
else
    echo "Server: [127.0.0.1:$1]"
    ./server/server -p "$1"
fi
