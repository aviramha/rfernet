extern crate fernet;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyBytes;


mod exc;

#[pyclass]
struct Fernet {
    fernet_: fernet::Fernet,
}


#[pymethods]
impl Fernet {
    #[new]
    fn new(obj: &PyRawObject, key: &str) -> PyResult<()> {
        match fernet::Fernet::new(key) {
            None => Err(exceptions::ValueError::py_err("Invalid arguments")),
            Some(fernet_obj) => {
                obj.init({
                    Fernet {
                        fernet_: fernet_obj,
                    }
                });
                Ok(())
            }
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
            Err(_err) => Err(exc::DecryptionError::py_err("Decryption failed, token or key invalid.")),
            Ok(data) => Ok(PyBytes::new(py, &data).into()),
        }
    }

    fn decrypt_with_ttl(&self, py: Python, token: &str, ttl_secs: u64) -> PyResult<PyObject> {
        match self.fernet_.decrypt_with_ttl(token, ttl_secs) {
            Err(_err) => Err(exc::DecryptionError::py_err("Decryption failed, token or key invalid.")),
            Ok(data) => Ok(PyBytes::new(py, &data).into()),
        }
    }

}



/// This module is a python module implemented in Rust.
#[pymodule]
fn rfernet(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fernet>()?;
    m.add("DecryptionError", py.get_type::<exc::DecryptionError>())?;
    Ok(())
}
