# rfernet
Python extension for Fernet encryption/decryption, faster than other alternatives.
This library uses the rust library `fernet-rs` https://github.com/mozilla-services/fernet-rs.

CI & Building wheels copied from `cryptography` and `orjson`

# Benchmark
Compared to cryptography's Fernet (CPU):
```
In [2]: from cryptography.fernet import Fernet as cFernet

In [3]: from rfernet import Fernet as rFernet

In [4]:

In [4]: plain = b"asd" * 1000

In [5]: key = rFernet.generate_new_key()

In [7]: r_fernet = rFernet(key)

In [8]: c_fernet = cFernet(key)

In [9]: %timeit r_fernet.encrypt(plain)
18.4 µs ± 117 ns per loop (mean ± std. dev. of 7 runs, 100000 loops each)

In [10]: %timeit c_fernet.encrypt(plain)
77.7 µs ± 921 ns per loop (mean ± std. dev. of 7 runs, 10000 loops each)
```
Memory:
```
# rfernet
[ Top 10 ]
<frozen importlib._bootstrap>:219: size=4444 B, count=38, average=117 B
test2.py:4: size=576 B, count=1, average=576 B
<frozen importlib._bootstrap_external>:59: size=156 B, count=1, average=156 B
test2.py:6: size=93 B, count=1, average=93 B
<frozen importlib._bootstrap>:371: size=80 B, count=1, average=80 B
<frozen importlib._bootstrap>:105: size=72 B, count=1, average=72 B
<frozen importlib._bootstrap_external>:1352: size=56 B, count=1, average=56 B
<frozen importlib._bootstrap_external>:606: size=56 B, count=1, average=56 B
test2.py:7: size=48 B, count=1, average=48 B
<frozen importlib._bootstrap_external>:1030: size=40 B, count=1, average=40 B

# cryptography's Fernet
[ Top 10 ]
<frozen importlib._bootstrap_external>:525: size=3134 KiB, count=31814, average=101 B
/Library/Frameworks/Python.framework/Versions/3.7/lib/python3.7/site-packages/cryptography/hazmat/bindings/openssl/binding.py:91: size=449 KiB, count=3169, average=145 B
<frozen importlib._bootstrap>:219: size=404 KiB, count=3384, average=122 B
/Library/Frameworks/Python.framework/Versions/3.7/lib/python3.7/abc.py:126: size=146 KiB, count=717, average=209 B
/Library/Frameworks/Python.framework/Versions/3.7/lib/python3.7/site-packages/cryptography/hazmat/bindings/openssl/binding.py:89: size=119 KiB, count=1773, average=69 B
/Library/Frameworks/Python.framework/Versions/3.7/lib/python3.7/abc.py:127: size=68.7 KiB, count=447, average=157 B
/Library/Frameworks/Python.framework/Versions/3.7/lib/python3.7/inspect.py:2793: size=46.8 KiB, count=282, average=170 B
<frozen importlib._bootstrap_external>:59: size=41.7 KiB, count=265, average=161 B
/Library/Frameworks/Python.framework/Versions/3.7/lib/python3.7/abc.py:135: size=40.8 KiB, count=339, average=123 B
/Library/Frameworks/Python.framework/Versions/3.7/lib/python3.7/site-packages/idna/idnadata.py:826: size=36.7 KiB, count=3, average=12.2 KiB
```
Memory test source code:
```
import tracemalloc

tracemalloc.start()
from cryptography.fernet import Fernet as cFernet
plain = b"asd" * 1000
key = cFernet.generate_key()
c_fernet = cFernet(key)
c_fernet.encrypt(plain)

snapshot = tracemalloc.take_snapshot()
top_stats = snapshot.statistics('lineno')

print("[ Top 10 ]")
for stat in top_stats[:10]:
    print(stat)
```
