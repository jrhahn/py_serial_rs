#! /usr/bin/env python

from py_rust import PySerial


import time


def current_milli_time():
    return round(time.time() * 1000)

serial = PySerial(460800, "/dev/ttyS0")


while True:
    try:
        buffer = serial.read_line(5000)
    except Exception as e:
        print(f"buffer not ready: {e}")
        continue;

    timestamp = current_milli_time()
    data = "".join(buffer).replace(" ", "")

    print(f"{timestamp} -> {data}")

    try:
        serial.write(b"from nixos\n")
        print("write succes")
    except:
        print("Writing failed")
        
