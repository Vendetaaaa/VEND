# VEND

VEND is a register-aliased assembly language for low-level systems work.

It keeps the machine-level feel of assembly, but adds lexical scopes, explicit register binding, strict operand widths, and structured control flow so the compiler can catch the mistakes that usually show up late.

If you want the short version / segments:

- [Why VEND exists](docs/overview.md)
- [What problems it solves](docs/overview.md#what-vend-solves)
- [How the project is organized](docs/roadmap.md)
- [Language spec and examples](docs/spec.md)
- [Example programs](docs/examples.md)

## Current Status

This repository currently holds the stage 1 language model:

- AST types for programs, functions, blocks, instructions, operands, and control flow
- the full x86_64-style physical register set used by the language model
- register width types for `reg64`, `reg32`, `reg16`, and `reg8`
- memory-address modeling for bracketed addressing expressions
- a grammar reference string for the language surface
- tests that build nested ASTs directly in memory

## Install, build, run
/

## License

This project uses the MIT License. See [LICENSE](LICENSE).
