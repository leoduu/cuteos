#!/bin/bash

sd_path=/dev/sdc

sync
sudo dd if=../tools/idbloader.img  of=$sd_path seek=64      #0x40
sudo dd if=../gboot.img            of=$sd_path seek=16384   #0x4000
sudo dd if=../tools/trust.img      of=$sd_path seek=24576   #0x6000
echo "|"
echo "|  Wait for sync"
echo "|"
sync
