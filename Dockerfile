FROM ghcr.io/pyo3/maturin

RUN yum install -y libudev-devel

WORKDIR /io

ENTRYPOINT ["bash"]
