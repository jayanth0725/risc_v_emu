# Rusty RISC-V Emulator

A bare-metal, 32-bit RISC-V CPU emulator written entirely in Rust from scratch.

This project simulates a RISC-V processor (RV32I base integer instruction set) complete with a simulated 1MB memory array, Memory-Mapped I/O (MMIO), and a proxy-kernel capable of handling 'ecall' system traps.

It can natively execute bare-metal C programs compiled with a RISC-V toolchain.

## Features
* **Full RV32I Decoding:** Supports all **base** R, I, S, B, U and J type instructions.
* **Bare-Metal C Execution:** Executes standard compiled ELF binaries (stripped via `objcopy`).
* **Memory-Mapped I/O:** Intercepts writes to `0x10000000` to simulate a physical terminal screen.
* **Proxy Kernel:** Implements `ecall` (Syscall 64 for `write` and Syscall 93 for `exit`).

## Prerequisites (Fedora / Linux)
You need Rust and the RISC-V cross-compiler toolchain to build C programs for the emulator.

### Linux (Fedora):
```bash
sudo dnf install rust cargo
sudo dnf install gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu
```
### Linux (Ubuntu/Debian):
```bash
sudo apt install rustc cargo
sudo apt install gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu
```

### macOS:
```bash
brew install rust
brew install riscv-gnu-toolchain
```

**Windows:** It is highly recommended to use WSL (Windows Subsystem for Linux) and follow the Ubuntu instructions above.

## Quick Start
1. **Compile the C program:** The provided `main.c` contains a test program that uses MMIO and Syscalls. Use the `Makefile` to compile it into a raw RISC-V binary.
```bash
make
```
2. **Run the Emulator:** Use Cargo to compile and execute the CPU in optimised release mode for maximum performance.
```bash
cargo run --release
```

## Acknowledgments
This emulator was built as a learning project to understand RISC-V architecture and Rust. I utilised an LLM to help navigate the RISC-V unprivileged specification (The Base Integer ISA, Registers x0-x31, and Instruction formats), bitwise decoding logic, and Rust's syntax and type-casting rules. The C-to-bare-metal cross-compilation pipeline and testing environment were built and configured manually on Fedora Linux.