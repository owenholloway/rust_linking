#!/usr/bin/env bash
gcc -c test_utils.c -o test_utils.o
ar rcs libtest_utils.a test_utils.o
mv ./libtest_utils.a ../../clib
rm ./*.o