#! /usr/bin/env python

import logging
import socket
import time

from py_rust import PySerial


def current_milli_time():
    return round(time.time() * 1000)


def run() -> None:
    logging.basicConfig(level=logging.INFO)
    logger = logging.getLogger(__name__)
    serial = PySerial(
        baud_rate=460800,
        port="/dev/ttyS0",
    )

    # message = f"{socket.gethostname()}"
    message = "123\456"
    while True:
        try:
            buffer = serial.read_line(
                timeout_in_millis=5000,
            )
            timestamp = current_milli_time()
            data = "".join(buffer).replace(" ", "")

            logger.info(f"{timestamp}: {data}")

        except Exception as e:
            logger.info(f"Reading failed: {e}")

        try:
            serial.write(f"{message}\r\n".encode())
        except Exeption as e:
            logger.error(f"Failed to write: {e}")


if __name__ == "__main__":
    run()
