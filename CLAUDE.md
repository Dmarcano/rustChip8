# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a CHIP-8 interpreter/emulator written in Rust with multiple frontend implementations:

1. **Core Library** (`src/`): The main CHIP-8 CPU implementation
2. **WebAssembly Frontend** (`wasm-chip8/`): Browser-based implementation using wasm-bindgen
3. **Web Game Engine Frontend** (`web_ez_impl/`): Native implementation using good-web-game/ggez

The project follows the CHIP-8 specification from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

## Development Commands

### WebAssembly Build
The primary working implementation uses WebAssembly:

```bash
# Build the WebAssembly package
cd wasm-chip8
wasm-pack build

# Run the web frontend
cd www
npm install  # if first time
npm run start
```

### Core Library Testing
```bash
# Run tests for the core library
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Web Game Engine Implementation
```bash
# Build and run the ggez implementation
cd web_ez_impl
cargo run
```

## Architecture

### Core CPU (`src/lib.rs`)
- `Chip8CPU` struct: Main interpreter with 4KB memory, 16 registers, stack, timers
- Opcode execution uses function table pattern for performance
- Display buffer: 64x32 monochrome pixels
- Built-in fontset for hexadecimal characters

### Opcode Implementation (`src/opcodes/mod.rs`)
- All 35 CHIP-8 opcodes implemented as methods
- Function table dispatch (`src/opcodes/function_table.rs`) maps opcodes to implementations
- Comprehensive test coverage for opcode behavior

### Frontends
- **WebAssembly**: Uses `web-sys` for canvas rendering and input handling
- **ggez**: Native game engine with graphics and audio capabilities
- Both frontends consume the same core `Chip8CPU` library

### Memory Layout
- `0x000-0x1FF`: Reserved (interpreter area)
- `0x200-0xFFF`: Program/data space (3584 bytes)
- Fontset loaded at `0x50-0x9F`

## Key Files
- `src/lib.rs`: Core CPU implementation and public API
- `src/opcodes/mod.rs`: All opcode implementations with extensive tests
- `src/dissassembler.rs`: Opcode disassembly for debugging
- `wasm-chip8/src/lib.rs`: WebAssembly bindings and web interface
- `web_ez_impl/src/main.rs`: Native game engine implementation

## Testing
The core library has comprehensive unit tests covering:
- Individual opcode behavior
- Register operations (arithmetic, logical, shifting)
- Memory operations and sprite drawing
- Stack operations and subroutine calls
- Display buffer collision detection

### Flag Register Tests (Based on Timendus CHIP-8 Test Suite)
Comprehensive flag tests that verify critical VF register behavior:
- **Bitwise operations**: Ensures VF is not modified by OR, AND, XOR operations
- **VF as operand**: Tests using VF as both input and output register
- **Shifting operations**: Verifies correct bit extraction to VF for SHR/SHL
- **Subtraction edge cases**: Tests both SUB and SUBN flag behavior
- **Early flag prevention**: Ensures VF operand values are read before flag updates

These tests are based on the Timendus CHIP-8 test suite and catch subtle implementation errors in flag management across arithmetic and logical instructions.

Use `cargo test` in the root directory to run all core library tests.
Use `cargo test flags_test` to run only the comprehensive flag behavior tests.