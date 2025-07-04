## Overview

`rust-emulas` is a Rust-based NES emulator project. It provides a modular and robust framework for building emulation software with performance and reliability in mind.

## Features

- Modular architecture for easy customization.
- High-performance emulation capabilities.
- Written entirely in Rust for safety and speed.

## Project Structure

```
rust-emulas/
├── .gitignore                  # Git ignore rules
├── Cargo.lock                  # Cargo lock file for reproducible builds
├── Cargo.toml                  # Project dependencies and metadata
├── mamaco.nes                  # Example NES ROM file for testing
├── README.md                   # Project documentation
├── Release.toml                # Release configuration
├── .github/
│   ├── pull_request_template.md    # Template for pull requests
│   └── workflows/
│       └── ci-release.yml         # CI workflow for releases
├── src/
│   ├── lib.rs                    # Library entry point (shared logic)
│   ├── main.rs                   # Binary entry point (CLI, loads ROM, runs emulator)
│   ├── cpu/
│   │   ├── flags.rs              # CPU status flag definitions and helpers
│   │   ├── instruction.rs        # CPU instruction set and decoding logic
│   │   └── mod.rs                # CPU module root, integrates CPU components
│   ├── memory/
│   │   └── mod.rs                # Memory bus and mapping logic
│   └── sys/
│       ├── errors.rs             # Custom error types for system operations
│       ├── interfaces.rs         # Traits and interfaces for system components and ROM abstraction
│       ├── mod.rs                # System module root
│       └── rom_file.rs           # NES ROM file parsing, validation, and memory mapping
└── target/
    └── ... (build artifacts)
```

### File Descriptions

- **src/main.rs**: The main executable. Handles CLI arguments, loads ROM files, and starts the emulation loop.
- **src/lib.rs**: Shared library code, re-exports core modules for use in both binary and tests.
- **src/cpu/**: Contains the CPU emulation logic.
  - **flags.rs**: Defines the CPU status flags and provides helper functions.
  - **instruction.rs**: Implements the instruction set, decoding, and execution logic.
  - **mod.rs**: Integrates CPU components and exposes the CPU interface.
- **src/memory/mod.rs**: Implements the memory bus, address mapping, and memory read/write logic.
- **src/sys/**: System-level abstractions and utilities.
  - **errors.rs**: Defines error types used throughout the emulator.
  - **interfaces.rs**: Contains traits and interfaces for system components and ROM abstraction, including:
    - `ROMFs`: Trait for ROM file operations (new, write_rom_memory, validate_file, read_file, read_exact_at, get_header, path, size)
    - `ROMFile`: Wrapper struct for ROMFs implementations
    - `INes`, `MirroringType`, `HeaderBytes`: NES ROM format structures
  - **rom_file.rs**: Handles loading, parsing, and validating NES ROM files, implements the `ROMFs` trait for the `ROM` struct, and provides functions for:
    - ROM file validation (`validate_file`)
    - Reading file content (`read_file`)
    - Reading ROM memory (`write_rom_memory`)
    - Reading exact bytes at offset (`read_exact_at`)
    - Extracting header (`get_header`)
    - Path and size accessors
  - **mod.rs**: Integrates system components.

## Getting Started

1. Clone the repository:
    ```bash
    git clone https://github.com/wesleyholiveira/rust-emulas.git
    cd rust-emulas
    ```

2. Build the project:
    ```bash
    cargo build
    ```

3. Run the project:
    ```bash
    cargo run
    ```

## Contributing

When making a commit, ensure that the version of the package in `Cargo.toml` is updated if necessary. Every commit should reflect the correct version of the package to maintain consistency and traceability.

Steps to verify:
1. Open the `Cargo.toml` file.
2. Check the `[package]` section for the `version` field.
3. Update the version if your changes introduce significant updates or fixes.
4. Include the updated `Cargo.toml` in your commit.

This ensures that the project versioning remains accurate and aligned with the changes made.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.