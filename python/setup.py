from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="engine_backend",
    version="0.1",
    rust_extensions=[RustExtension("engine_backend.engine_backend", "Cargo.toml")],
    packages=["engine_backend"],
    zip_safe=False,
)
