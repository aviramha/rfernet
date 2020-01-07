import time
import pytest

import rfernet



def test_sanity():
    key = rfernet.Fernet.generate_new_key()
    # Generates random string already so why not?
    plain = rfernet.Fernet.generate_new_key().encode()
    fernet = rfernet.Fernet(key)
    encrypted = fernet.encrypt(plain)
    assert fernet.decrypt(encrypted) == plain

    encrypted = fernet.encrypt(plain)
    assert fernet.decrypt_with_ttl(encrypted, 1000) == plain

def test_error_ttl():
    key = rfernet.Fernet.generate_new_key()
    # Generates random string already so why not?
    plain = rfernet.Fernet.generate_new_key().encode()
    fernet = rfernet.Fernet(key)

    encrypted = fernet.encrypt(plain)
    with pytest.raises(rfernet.DecryptionError):
        time.sleep(2)
        fernet.decrypt_with_ttl(encrypted, 1)

def test_invalid_key():
    with pytest.raises(ValueError):
        rfernet.Fernet("asd")

def test_decryption_failure():
    fernet_1 = rfernet.Fernet(rfernet.Fernet.generate_new_key())
    fernet_2 = rfernet.Fernet(rfernet.Fernet.generate_new_key())
    encrypted = fernet_1.encrypt(rfernet.Fernet.generate_new_key().encode())
    with pytest.raises(rfernet.DecryptionError):
        fernet_2.decrypt(encrypted)