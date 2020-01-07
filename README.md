# rfernet
Python extension for Fernet encryption/decryption, faster than other alternatives.
This library uses the rust library `fernet-rs` https://github.com/mozilla-services/fernet-rs.

# Benchmark
Compared to cryptography's Fernet:
```
In [1]: from cryptography.fernet import Fernet                                                                        

In [2]: from cryptography.fernet import Fernet as cFernet                                                             

In [3]: from rfernet import Fernet as rFernet                                                                         

In [4]:                                                                                                               

In [4]: plain = b"asd" * 1000                                                                                         

In [5]: key = rFernet.generate_new_key()                                                                              

In [6]: key                                                                                                           
Out[6]: 'ibvAZbdKzaqvcuWWbwDRH9-h7lc2aa_TWGHE9Peb2AI='

In [7]: r_fernet = rFernet(key)                                                                                       

In [8]: c_fernet = cFernet(key)                                                                                       

In [9]: %timeit r_fernet.encrypt(plain)                                                                               
18.4 µs ± 117 ns per loop (mean ± std. dev. of 7 runs, 100000 loops each)

In [10]: %timeit c_fernet.encrypt(plain)                                                                              
77.7 µs ± 921 ns per loop (mean ± std. dev. of 7 runs, 10000 loops each)
```
