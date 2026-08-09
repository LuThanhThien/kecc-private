# 1 Description

Implement the C AST printer in `src/c/write_c.rs`. The printed C must parse into an equivalent AST for all supported syntax.

# 2 Criteria

`make hw1` must exit successfully. It runs:

- Formatting: `cargo fmt --all -- --check`.
- Linting: `cargo clippy`.
- All 60 C examples.
- Seeded fuzzing: `--print -n80 --seed 22`.

# 3 Implementation

## 3.1 Test flow

`test_examples_write_c` in `tests/test_examples.rs` calls `test_write_c(path)` for each C file in `examples/c`.

`test_write_c` in `src/tests.rs` performs:

1. Parse the input C file into an AST (`unit`).
1. Print `unit` into `temp.c` using `write`.
1. Parse `temp.c` into another AST (`new_unit`).
1. Check `unit.is_equiv(&new_unit)`, ignoring source locations.

## 3.2 Parsing

`Parse.translate` in `src/c/parse.rs` calls `lang_c::driver::parse`. The library returns preprocessed source text and an AST. `Parse.translate` checks supported syntax and returns only the AST (`unit`).

## 3.3 Printer implementation

Implement `WriteLine` and `WriteString` in `src/c/write_c.rs`. Recursively print each supported AST node as C, preserving declaration order, operator grouping, and literal types.

## 3.4 Examples of AST structure

These examples illustrate how C source maps to AST nodes.

**Entire file:** `TranslationUnit(Vec<Node<ExternalDeclaration>>)` holds top-level declarations and function definitions.

```c
int x = 10;
int foo() { return x; }
int y;
```

```text
TranslationUnit
├── Declaration: int x = 10;
├── FunctionDefinition: int foo() { return x; }
└── Declaration: int y;
```

**Function definition:** for `int add(int a, int b) { return a + b; }`:

```text
FunctionDefinition
├── specifiers: int
├── declarator: add(int a, int b)
├── declarations: empty (used for old-style parameter declarations)
└── statement: { return a + b; }
```

`specifiers` is a vector because a declaration can have multiple specifiers, such as `unsigned long`.
