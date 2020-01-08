FROM pyca/cryptography-manylinux1:x86_64

RUN yum install -y gcc gcc-c++ make cmake
