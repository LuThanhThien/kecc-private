# Project Structure

This project is a small C compiler pipeline. The code is split by compilation stage and by homework topic.

## `src`

The main compiler code is under `src`.

- `src/c/`: C frontend; parses C source, validates supported syntax, and writes AST back to C.
- `src/ir/`: intermediate representation (IR), IR parsing/writing, and interpreter.
- `src/irgen/`: converts C AST into IR.
- `src/opt/`: optimization passes.
- `src/asm/`: assembly representation.
- `src/asmgen/`: converts IR to RISC-V assembly.
- `src/write_base.rs`: shared writing traits used by pretty-printers.
- `src/lib.rs`: project exports.

## `tests`

- `tests/test_examples.rs`: runs the example-based validation for each stage.
- `tests/fuzz.py`: fuzzing script for stress testing.
- `examples/`: sample C and IR programs used in grading and debugging.

## `bin`

Command-line tools and executables are in `bin`.

- `bin/kecc.rs`: main compiler command-line entry.
- `bin/fuzz.rs`: fuzzing runner for randomized testing.

## `scripts`

- `scripts/grade-*.sh`: grading scripts for each assignment stage, such as C writing, IR gen, optimization, and asm generation.

## Other folders

- `bench/`: benchmark programs.
- `csmith/`: CSmith random program generator used for testing.
- `docs/`: project notes and documentation.
