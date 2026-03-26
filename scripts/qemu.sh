#!/bin/sh

TEMP_DIR=$(mktemp -d)
ESP_IMG="$TEMP_DIR/esp.img"

fallocate -l 64M "$ESP_IMG"
mkfs.vfat -F 32 "$ESP_IMG"

mmd -i "$ESP_IMG" ::/EFI ::/EFI/BOOT ::/System
mcopy -i "$ESP_IMG" "$1" ::/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 \
    -bios "/usr/share/qemu/OVMF.fd" \
    -drive format=raw,file="$ESP_IMG",media=disk \
    -net none \
    -m 128\
    -d guest_errors \
    -D qemu.log \
    -serial none \
    -vga std \
    -s