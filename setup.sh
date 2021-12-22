#!/bin/sh

# Checking if Rust is installed
if ! [ -x "$(command -v cargo)" ] || ! [ -x "$(command -v rustc)" ]; then
    echo "Rust is not installed. Please follow the instructions on https://www.rust-lang.org/tools/install"
    exit
fi

echo "Creating pod root dir"
mkdir -p ./root
mkdir -p ./root/lib
mkdir -p ./root/lib64
mkdir -p ./root/bin

echo "Copying the bash executable"
cp /bin/bash ./root/bin/sh

echo "Copying the ls executable"
cp /bin/ls ./root/bin

echo "Building test app"
rustc ./src/test.rs -o ./root/test

echo "Building main app"
cargo build

echo ""
echo "All finished!"
echo "You can run 'sudo ./target/debug/podify sh' to start a shell in the pod"
echo "Or run 'sudo ./target/debug/podify /test' to run the test program"
