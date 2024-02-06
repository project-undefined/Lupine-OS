#!/usr/bin/env bash

cd ../os
BSP=rpi4 make
cp kernel8.img ../chainloader/demo_payload_rpi4.img
make
cp kernel8.img ../chainloader/demo_payload_rpi3.img
rm kernel8.img
