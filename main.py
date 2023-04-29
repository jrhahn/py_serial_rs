from  py_rust import PySerial

import time

def current_milli_time():
    return round(time.time() * 1000)

serial = PySerial(460800, "/dev/ttyS0")

while True:
    buffer = serial.read_line()

    #print(buffer)

    timestamp = current_milli_time()
    data = ''.join(buffer).replace(' ', '')

    print(f"{timestamp} -> {data}")
