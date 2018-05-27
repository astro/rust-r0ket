#!/usr/bin/env bash

set -e

cargo build --release

(
    set -e

    cd target/thumbv7m-none-eabi/release
    arm-none-eabi-objcopy -O binary --strip-unneeded bildfunk bildfunk.c0d
    ls -l bildfunk bildfunk.c0d

    sudo mount /dev/sdb /mnt/sd/
    sudo cp bildfunk.c0d /mnt/sd/
    sudo umount /mnt/sd/
    sync
    echo Copied
)
