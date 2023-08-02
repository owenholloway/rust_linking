#!/usr/bin/env bash
cargo +nightly clean
rm ./headers/*.h

cargo +nightly build --release

cp ./headers/*.h ./c/test/
cp ./target/release/*.a ./c/test/

cd ./c/test/
gcc -o my_program main.c -L. -lpoc_test -ltest_utils  -I. -ldl -lpthread -lm