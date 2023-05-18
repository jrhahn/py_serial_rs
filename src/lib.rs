use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::Python;
use std::str;
use std::time::Duration;
use std::time::SystemTime;

struct SerialReadResult<'a> {
    data: &'a [u8],
    is_complete: bool,
}

#[pyclass]
struct PySerial {
    serial: Box<dyn serialport::SerialPort>,
}

fn check_python_signals() -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> { py.check_signals() })
}

fn is_new_line(value: u8) -> bool {
    0x0a == value
}

fn copy_until_end_of_line(buffer_current: &[u8]) -> SerialReadResult {
    let mut len = buffer_current.len();

    for (i, v) in buffer_current.iter().enumerate() {
        if is_new_line(*v) {
            len = i;
            break;
        }
    }

    SerialReadResult {
        data: &buffer_current[..len],
        is_complete: len < buffer_current.len(),
    }
}

#[pymethods]
impl PySerial {
    #[new]
    fn connect(baud_rate: u32, port: &str) -> PyResult<PySerial> {
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

    fn read_line(&mut self, timeout_in_millis: u64) -> PyResult<String> {
        let time_start = SystemTime::now();
        let mut buffer: Vec<u8> = Vec::new();

        loop {
            check_python_signals()?;

            let mut buffer_current: Vec<u8> = vec![0, 32];

            if self.serial.read(buffer_current.as_mut_slice()).is_ok() {
                let serial_result = copy_until_end_of_line(&buffer_current);

                buffer.extend_from_slice(serial_result.data);

                if serial_result.is_complete {
                    break;
                }
            };

            if let Ok(time_elapsed) = time_start.elapsed() {
                if time_elapsed > Duration::from_millis(timeout_in_millis) {
                    return Err(exceptions::PyTimeoutError::new_err(
                        "Timeout occurred when trying to read",
                    ));
                }
            }
        }
        // serial_buf.iter().collect::<String>().as_bytes()

        let result = String::from_utf8(buffer)?;

        Ok(result)
    }

    fn write(&mut self, data: &[u8]) -> PyResult<usize> {
        Ok(self.serial.write(data)?)
    }

    fn close(&mut self) {}
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
