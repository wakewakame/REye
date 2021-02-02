#!/bin/sh

set -e

if [ "$1" = "debug" ]; then
	echo "debug"
	wasm-pack build --target web --debug
else
	echo "release"
	wasm-pack build --target web --release
fi

python3 -m http.server
