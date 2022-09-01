#!/usr/bin/bash

cargo build

cc -Wall -g -O0 main.c -I. -Ltarget/debug/ -lrevseq

LD_LIBRARY_PATH=target/debug ./a.out

