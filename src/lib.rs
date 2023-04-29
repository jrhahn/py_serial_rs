use pyo3::prelude::*;
use std::str;
use std::time::Duration;
use std::time::Instant;

// Example:
// https://saidvandeklundert.net/2021-11-18-calling-rust-from-python-using-pyo3/
#[pyclass]
struct PySerial {
    serial: Box<dyn serialport::SerialPort>,
}

#[pymethods]
impl PySerial {
    #[new]
    fn connect(baud_rate: u32, port: &str) -> PySerial {
        // fn connect(baud_rate: u32, port: &str) -> PyResult<Box<dyn serialport::SerialPort>> {
        let serial = serialport::new(port, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");

        PySerial { serial }
    }
    fn read_line(&mut self) -> Vec<char> {
        let mut serial_buf: Vec<char> = Vec::new();

        let mut done = false;

        while !done {
            let mut buf: Vec<u8> = vec![0, 32];
            self.serial
                .read(buf.as_mut_slice())
                .expect("Found no data!");

            for val in buf.iter() {
                let v = *val as char;
                serial_buf.push(v);
                if '\n' == v {
                    done = true;
                    break;
                }
            }
        }

        serial_buf
    }
}

#[pyfunction]
fn list_ports() {
    println!("Listing ports..");
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_ports, m)?)?;
    m.add_class::<PySerial>()?;
    Ok(())
}

// fn main() {
// list_ports();

// let mut serial = PySerial::connect(460_800, "/dev/ttyS0");

// let start = Instant::now();

// while true {
// let elapsed = start.elapsed();

// match port {
// Ok(v) => {
// let buffer = serial.read_line();

// println!("{}", elapsed.as_millis());
// }
// Err(e) => println!("Error {e:?}",),
// }
//println!("{} -> {:?}", elapsed.as_millis(), buffer);
// }
// }
