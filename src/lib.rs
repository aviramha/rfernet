extern crate fernet;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

mod exc;

#[pyclass]
struct Fernet {
    fernet_: fernet::Fernet,
}

#[pyclass]
struct MultiFernet {
    fernet_: fernet::MultiFernet,
}

#[pymethods]
impl Fernet {
    #[new]
    fn new(key: &str) -> PyResult<Self> {
        match fernet::Fernet::new(key) {
            None => Err(exceptions::PyValueError::new_err("Invalid arguments")),
            Some(fernet_obj) => Ok(Fernet {
                fernet_: fernet_obj,
            }),
        }
    }

    fn encrypt(&self, data: &[u8]) -> PyResult<String> {
        Ok(self.fernet_.encrypt(data))
    }

    #[staticmethod]
    fn generate_new_key() -> PyResult<String> {
        Ok(fernet::Fernet::generate_key())
    }

    fn decrypt(&self, py: Python, token: &str) -> PyResult<PyObject> {
        match self.fernet_.decrypt(token) {
            Err(_err) => Err(exc::DecryptionError::new_err(
                "Decryption failed, token or key invalid.",
            )),
            Ok(data) => Ok(PyBytes::new(py, &data).into()),
        }
    }

    fn decrypt_with_ttl(&self, py: Python, token: &str, ttl_secs: u64) -> PyResult<PyObject> {
        match self.fernet_.decrypt_with_ttl(token, ttl_secs) {
            Err(_err) => Err(exc::DecryptionError::new_err(
                "Decryption failed, token or key invalid.",
            )),
            Ok(data) => Ok(PyBytes::new(py, &data).into()),
        }
    }
}

#[pymethods]
impl MultiFernet {
    #[new]
    fn new(keys: Vec<&str>) -> PyResult<Self> {
        let fernets: Option<Vec<_>> = keys.iter().map(|&k| fernet::Fernet::new(k)).collect();
        match fernets {
            None => Err(exceptions::PyValueError::new_err("Invalid arguments")),
            Some(f) => Ok(MultiFernet {
                fernet_: fernet::MultiFernet::new(f),
            }),
        }
    }

    fn encrypt(&self, data: &[u8]) -> PyResult<String> {
        Ok(self.fernet_.encrypt(data))
    }

    fn decrypt(&self, py: Python, token: &str) -> PyResult<PyObject> {
        match self.fernet_.decrypt(token) {
            Err(_err) => Err(exc::DecryptionError::new_err(
                "Decryption failed, token or key invalid.",
            )),
            Ok(data) => Ok(PyBytes::new(py, &data).into()),
        }
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rfernet(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fernet>()?;
    m.add_class::<MultiFernet>()?;
    m.add("DecryptionError", py.get_type::<exc::DecryptionError>())?;
    Ok(())
}
