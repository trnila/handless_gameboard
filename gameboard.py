#!/usr/bin/env python3
import argparse
from pynput.keyboard import Key, Controller
import serial
from mappings import mappings

par = argparse.ArgumentParser()
par.add_argument('--serial', default='/dev/ttyACM0')
par.add_argument('mapping', choices=mappings.keys())
args = par.parse_args()

keyboard = Controller()
s = serial.Serial(args.serial, 9600)
mapping = mappings[args.mapping]

while True:
    x = s.read(1)[0]

    key = x & 0x7f
    press = x & 0x80

    seq = mapping.get(key, [])

    for key in seq:
        if press:
            keyboard.press(key)
        else:
            keyboard.release(key)

