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


def test_multifernet_sanity():
    keys = [rfernet.Fernet.generate_new_key() for _ in range(6)]

    ferns = [rfernet.Fernet(k) for k in keys]

    mfern = rfernet.MultiFernet(keys)

    for encryptor in ferns:
        cypher = encryptor.encrypt(b'hello there')
        decyphered = mfern.decrypt(cypher)
        assert decyphered == b'hello there'


def test_multifernet_enc():
    keys = [rfernet.Fernet.generate_new_key() for _ in range(6)]
    fern = rfernet.Fernet(keys[0])

    mfern = rfernet.MultiFernet(keys)

    for plaintext in [
        b'hello there',
        b'',
        b'why'
    ]:
        single_cypher = fern.encrypt(plaintext)
        multi_cypher = mfern.encrypt(plaintext)
        assert mfern.decrypt(single_cypher) == fern.decrypt(multi_cypher) == plaintext


def test_mfern_invalid_key():
    with pytest.raises(ValueError):
        rfernet.MultiFernet([rfernet.Fernet.generate_new_key(), "asd", rfernet.Fernet.generate_new_key()])


def test_mfern_decryption_failure():
    keys = [rfernet.Fernet.generate_new_key() for _ in range(6)]

    fern = rfernet.Fernet(rfernet.Fernet.generate_new_key())

    mfern = rfernet.MultiFernet(keys)

    cypher = fern.encrypt(b'hello there')
    with pytest.raises(rfernet.DecryptionError):
        mfern.decrypt(cypher)
