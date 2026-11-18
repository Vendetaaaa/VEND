# Roadmap

VEND is being built in stages so the language can stay coherent as it grows.

## Stage 1: AST and Grammar [x]

Define the core syntax tree, the register model, and the initial grammar.

## Stage 2: Register Overlap [x]

Model physical register overlap accurately so aliases cannot silently conflict.

## Stage 3: Parser and Diagnostics [x]

Parse source files and report errors in a way that points directly to the problem.

## Stage 4: Scope and Liveness [x]

Track lexical register lifetimes and block boundaries.

## Stage 5: Type Checking [x]

Check alias widths, register-to-register operand widths, condition widths, and
the x86-64 memory-address register and scale forms represented by the AST.

## Stage 6: Control-Flow Lowering [x]

Convert structured `if` and `while` constructs into deterministic low-level branches.

## Stage 7: Assembly Emission [x]

Emit valid x86_64 assembly for supported toolchains.

## Stage 8: ABI Boundaries [x]

Support external call symbols and expose the System V and Windows x64 register,
stack-alignment, shadow-space, return-register, and caller-saved rules.

## Stage 9: Unsafe Idioms [x]

Support explicit `unsafe { ... }` blocks for hand-tuned low-level patterns.

## Stage 10: CLI and Tooling [x]

The `vend` command-line binary parses and validates a source file, lowers
structured control flow, and emits Intel-syntax x86-64 assembly. It supports
`--abi systemv|windows` and `-o/--output`.
