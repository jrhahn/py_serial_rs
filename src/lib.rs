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

fn copy_until_end_of_line(buffer_current: &[u8], num_bytes_read: usize) -> SerialReadResult {
    let mut len = buffer_current.len();

    let mut is_complete = false;

    for (i, v) in buffer_current.iter().enumerate() {
        if is_new_line(*v) {
            len = i;
            is_complete = true;
            break;
        }

        if i >= num_bytes_read {
            len = num_bytes_read;
            break;
        }
    }

    SerialReadResult {
        data: &buffer_current[..len],
        is_complete,
    }
}

fn _read_line(
    serial: &mut Box<dyn serialport::SerialPort>,
    timeout_in_millis: Option<u64>,
) -> PyResult<String> {
    let time_start = SystemTime::now();
    let mut buffer: Vec<u8> = Vec::new();

    loop {
        check_python_signals()?;

        let mut buffer_current: Vec<u8> = vec![0, 32];

        if let Ok(num_bytes_read) = serial.read(buffer_current.as_mut_slice()) {
            let serial_result = copy_until_end_of_line(&buffer_current, num_bytes_read);

            buffer.extend_from_slice(serial_result.data);

            if serial_result.is_complete {
                break;
            }
        };

        if let Some(timeout) = timeout_in_millis.unwrap() {
            if let Ok(time_elapsed) = time_start.elapsed() {
                if time_elapsed > Duration::from_millis(timeout) {
                    return Err(exceptions::PyTimeoutError::new_err(
                        "Timeout occurred when trying to read",
                    ));
                }
            }
        }
    }

    let result = String::from_utf8(buffer)?;

    Ok(result)
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
        Python::with_gil(|py| -> PyResult<String> {
            py.allow_threads(move || _read_line(&mut self.serial, timeout_in_millis))
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_new_line_negative() {
        assert!(!is_new_line(0))
    }

    #[test]
    fn is_new_line_positive() {
        assert!(is_new_line(0x0a))
    }

    #[test]
    fn copy_until_end_of_line_incomplete() {
        let actual = copy_until_end_of_line(&[0], 1);
        let expected = [0];

        assert!(!actual.is_complete);
        assert_eq!(expected.len(), actual.data.len());

        for ii in 0..expected.len() {
            assert_eq!(expected[ii], actual.data[ii]);
        }
    }

    #[test]
    fn copy_until_end_of_line_complete() {
        let actual = copy_until_end_of_line(&[0, 12, 13, 10, 12], 5);
        let expected = [0, 12, 13];

        assert!(actual.is_complete);
        assert_eq!(
            expected.len(),
            actual.data.len(),
            "Data has unexpected length."
        );

        for ii in 0..expected.len() {
            assert_eq!(expected[ii], actual.data[ii]);
        }
    }
}
