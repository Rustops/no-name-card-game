#!/bin/bash

cargo build --release

cp ./target/release/no-name-card-game ./client/

if [ -z "$1" ] && [ -z "$2" ] && [ -z "$3" ] ;then
    echo "Use the default username and server address."
    echo "Username: [Client] - Server: [127.0.0.1:6666]"
    ./client/no-name-card-game
else
    echo "Username: [$1] - Server: [$2] - Udp Port: [$3]"
    ./client/no-name-card-game --name "$1" --url "$2" --port "$3"
fi
