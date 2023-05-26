.. py_serial_rs documentation master file, created by
   sphinx-quickstart on Sun May 21 10:31:33 2023.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

py_serial_rs
========================================

The goal of this package is to provide a fast and reliable interface to the serial ports.

Instead of implementing the logic in Python, this package builds around `serialport <https://docs.rs/serialport/latest/serialport/>`_ written in Rust. This package supports threading.


========================================
Installation
========================================
`PyO3 <https://pyo3.rs/v0.18.3/>`_ (with maturin) will build a pip package for you. For this, make
sure that you are able to build rust packages with `cargo`. Create a virtual environment
and install the required pip packages with

.. code-block:: bash

   pip install -r requirements.txt

The pip package can then easily be build with

.. code-block:: bash

   maturin build --release

Voilà, a pip package to be used in your code!


========================================
Demo Scripts
========================================
Demo scripts are located in `/python`:

* `demo.py` shows a simple direct communication via the serial connection
* `demo_threads.py` demonstrates how this package can be used with threads

.. toctree::
   :maxdepth: 2
   :caption: Contents:

========================================
API
========================================
.. automodule:: py_serial_rs
   :members:
   :undoc-members:
   :private-members:
