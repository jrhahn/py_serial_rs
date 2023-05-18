#! /usr/bin/env python

import logging
import socket
import time
from threading import Thread

from py_rust import PySerial


def current_milli_time():
    return round(time.time() * 1000)


def run_counter():
    counter = 0

    while True:
        print(f"Counter {counter}")
        counter += 1


def run_serial() -> None:
    logging.basicConfig(level=logging.INFO)
    logger = logging.getLogger(__name__)
    serial = PySerial(
        baud_rate=460800,
        port="/dev/ttyS0",
    )

    message = f"{socket.gethostname()}"
    while True:
        try:
            data = serial.read_line(
                timeout_in_millis=5000,
            )

            timestamp = current_milli_time()

            logger.info(f"{timestamp}: {data}")

        except Exception as e:
            logger.info(f"Reading failed: {e}")

        try:
            serial.write(f"{message}\n".encode())
        except Exeption as e:
            logger.error(f"Failed to write: {e}")


def run() -> None:
    thread0 = Thread(
        target=run_counter,
        daemon=True,
    )

    thread0.start()

    thread1 = Thread(
        target=run_serial,
        daemon=True,
    )

    thread1.start()
    thread0.join()
    thread1.join()


if __name__ == "__main__":
    run()
