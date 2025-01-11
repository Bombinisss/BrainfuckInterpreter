[![dependency status](https://deps.rs/repo/github/bombinisss/BrainfuckInterpreter/status.svg)](https://deps.rs/repo/github/bombinisss/BrainfuckInterpreter)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Bombinisss/BrainfuckInterpreter/blob/master/LICENSE-MIT.txt)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/Bombinisss/BrainfuckInterpreter/blob/master/LICENSE-APACHE.txt)

# Brainfuck Interpreter written in Rust

- **Program Description**: This Brainfuck interpreter displays the program's memory state and execution step by step.
- **Visual Memory Representation**: The grid shown in the image represents the memory of the Brainfuck program.
- **Execution Pointer**: The red cell highlights the memory location currently being executed by the interpreter.
- **Memory Clear**: Cells/Memory is reset evey time the program is run.
- **Memory Out Of Bounds auto fix**: When program tries to use more memory than allocated, more is allocated.
- **Values Out Of Bounds auto fix**: When trying to add more than 255 it goes bac to 0 and in reverse too.

![image](https://github.com/user-attachments/assets/640f9168-6743-4a74-ad13-543c7455f136)

https://github.com/user-attachments/assets/fcd72b7f-2ad9-455c-ae99-d200a0ea9d3e


## What is Brainfuck?

**Brainfuck** is a minimalistic esoteric programming language, notable for its extreme simplicity, small size, and almost cryptic syntax. It was created in 1993 by Urban Müller and is designed to challenge and amuse programmers rather than being a practical language for large-scale software development. The language consists of only eight commands, each represented by a single character:

- `>`: Move the pointer to the right
- `<`: Move the pointer to the left
- `+`: Increment the byte at the pointer
- `-`: Decrement the byte at the pointer
- `.`: Output the byte at the pointer as an ASCII character
- `,`: Accept one byte of input, storing its value at the pointer
- `[`: If the byte at the pointer is zero, jump forward past the matching `]`
- `]`: If the byte at the pointer is nonzero, jump backward to the matching `[`

Despite its simplicity, Brainfuck is Turing-complete, meaning it can theoretically compute any computable function given enough time and memory. Its code often appears as a collection of `+`, `-`, `<`, `>`, `[`, `]`, `.`, and `,`, resulting in programs that look extremely terse and obfuscated. This makes Brainfuck a favorite tool for code golfers, obfuscators, and enthusiasts who enjoy pushing languages to their limits.

---

### Build

**Prerequisite:** Have Rust installed!

**Build:**
```bash
cargo build --release
```

**Run:**
```bash
cargo run --release
```

### Tests

To run tests 
```bash
cargo test
```


 
### Dependencies

Made using [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).
