#!/bin/bash

BIN=$1
riscv64-elf-objcopy -O binary $BIN "$BIN.bin"
dfu-suffix --add "$BIN.bin"
dfu-util -a 0 -d 28e9:0189 -D "$BIN.bin" --dfuse-address 0x08000000:leave
