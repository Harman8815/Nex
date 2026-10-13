NEX — Native Execution eXperiment “Beyond the source.”

We'll use `nex` as the repository and Cargo package name, `.nex` as the custom source-file extension, and NEX throughout the README, compiler, and project structure.

Example:

```
nex/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   ├── parser.rs
│   ├── ast.rs
│   ├── semantic.rs
│   └── codegen.rs
├── programs/
│   └── main.nex
├── tests/
└── output/
```

The initial compilation pipeline remains:

NEX source (`.nex`) → Rust-based compiler → generated Rust (`.rs`) → `rustc` → executable.