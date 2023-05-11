#! /usr/bin/env python

from  py_rust import PySerial


import time

def current_milli_time():
    return round(time.time() * 1000)

serial = PySerial(115200, "/dev/ttyS0")

while True:
    try:
        buffer = serial.read_line()
    except:
        print("buffer not ready")
        continue;
    #print(buffer)

    timestamp = current_milli_time()
    data = ''.join(buffer).replace(' ', '')

    print(f"{timestamp} -> {data}")
