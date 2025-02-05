# 2 Lang

The worlds most efficient systems programming language

- [Language Docs](docs/language.md)
- [Compiler Docs](docs/compiler.md)
- [Docgen Spec](docs/docgen.md)

## Theory

2 Lang is built upon three simple ideas

- Writing the number 0 will write an **unset** bit (0) to a file
- Writing the number 1 will write a **set** bit (1) to a file
- You can repeat a template of 1s and 0s using a macro
  - `#(MY::MACRO) 1101` will write the bits 1101 whenever `(MY::MACRO)` is written
