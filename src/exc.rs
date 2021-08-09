use pyo3::create_exception;

create_exception!(rfernet, DecryptionError, pyo3::exceptions::PyTypeError);
