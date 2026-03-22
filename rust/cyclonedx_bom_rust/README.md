# cyclonedx-bom-rust

Optional Rust accelerator for `cyclonedx-bom` JSON validation (including JSON Schema `format` checks).

## Install

End users (no Rust toolchain required when wheels are available):

```sh
python -m pip install 'cyclonedx-bom[speedups]'
# or
python -m pip install cyclonedx-bom-rust
```

## Build & install (dev)

Prereqs:

- Rust toolchain (`rustc`, `cargo`)
- Python (>=3.9)

From the repo root:

```sh
python -m pip install -U maturin
maturin develop -m rust/cyclonedx_bom_rust/pyproject.toml
```

After installing, `cyclonedx-py` will automatically use the Rust validator for JSON output.
