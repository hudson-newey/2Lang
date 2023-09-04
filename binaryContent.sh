#!/bin/sh
xxd -b -c 1 ./a.out | awk '{print $2}' | tr -d '\n'
