use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::Python;
use std::str;
use std::time::Duration;
use std::time::SystemTime;

#[pyclass]
struct PySerial {
    serial: Box<dyn serialport::SerialPort>,
}

fn check_python_signals() -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> { py.check_signals() })
}

#[pymethods]
impl PySerial {
    fn write(&mut self, data: &[u8]) -> PyResult<usize> {
        Ok(self.serial.write(data)?)
    }

    fn close(&mut self) {}

    #[new]
    fn connect(baud_rate: u32, port: &str) -> PyResult<PySerial> {
        // fn connect(baud_rate: u32, port: &str) -> PyResult<Box<dyn serialport::SerialPort>> {
        match serialport::new(port, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(serial) => Ok(PySerial { serial }),
            Err(error) => Err(exceptions::PyTypeError::new_err(format!(
                "Failed to open port {port}: {error}"
            ))),
        }
    }

    fn read_line(&mut self, timeout_in_millis: u64) -> PyResult<Vec<char>> {
        let mut serial_buf: Vec<char> = Vec::new();

        let mut done = false;

        let time_start = SystemTime::now();

        while !done {
            check_python_signals()?;

            let mut buf: Vec<u8> = vec![0, 32];

            match self.serial.read(buf.as_mut_slice()) {
                Ok(_) => {
                    for val in buf.iter() {
                        let v = *val as char;
                        serial_buf.push(v);
                        if '\n' == v || '\r' == v || 'a' == v {
                            done = true;
                            break;
                        }
                    }
                }
                Err(_) => {}
            };

            if let Ok(time_elapsed) = time_start.elapsed() {
                if time_elapsed > Duration::from_millis(timeout_in_millis) {
                    // todo raise Exception
                    return Err(exceptions::PyTimeoutError::new_err(format!(
                        "Timeout occurred when trying to reading",
                    )));
                }
            }
        }

        Ok(serial_buf)
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

#[pymodule]
fn py_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_ports, m)?)?;
    m.add_class::<PySerial>()?;
    Ok(())
}
