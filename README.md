# VEND

VEND is a register-aliased assembly language for low-level systems work.

It keeps the machine-level feel of assembly, but adds lexical scopes, explicit register binding, register-width modeling, and structured control flow so the compiler can catch the mistakes that usually show up late.

If you want the short version / segments:

- [Why VEND exists](docs/overview.md)
- [What problems it solves](docs/overview.md#what-vend-solves)
- [How the project is organized](docs/roadmap.md)
- [Language spec and examples](docs/spec.md)
- [Example programs](docs/examples.md)

## Current Status

# VEND

VEND is a register-aliased assembly language for low-level systems work.

It keeps the machine-level feel of assembly, but adds lexical scopes, explicit register binding, register-width modeling, and structured control flow so the compiler can catch the mistakes that usually show up late.

If you want the short version / segments:

- [Why VEND exists](docs/overview.md)
- [What problems it solves](docs/overview.md#what-vend-solves)
- [How the project is organized](docs/roadmap.md)
- [Language spec and examples](docs/spec.md)
- [Example programs](docs/examples.md)

## Current Status

Stages 1 through 5 are implemented:

- AST types for programs, functions, blocks, instructions, operands, and control flow
- the full x86_64-style physical register set used by the language model
- register width types for `reg64`, `reg32`, `reg16`, and `reg8`
- memory-address modeling for bracketed addressing expressions
- a grammar reference string for the language surface
- register-overlap validation for aliases
- a lexer, recursive-descent parser, and source-pointing diagnostics
- lexical scope metadata, alias lifetimes, use tracking, and visibility diagnostics
- register-width validation for aliases, register-to-register instructions, and conditions
- memory-address validation for 64-bit base/index registers and legal index scales
- unit and integration tests for the complete foundation

## Install, build, run
/

## License

This project uses the MIT License. See [LICENSE](LICENSE).
