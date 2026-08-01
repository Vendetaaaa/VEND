# Roadmap

VEND is being built in stages so the language can stay coherent as it grows.

## Stage 1: AST and Grammar

Define the core syntax tree, the register model, and the initial grammar.

## Stage 2: Register Overlap

Model physical register overlap accurately so aliases cannot silently conflict.

## Stage 3: Parser and Diagnostics

Parse source files and report errors in a way that points directly to the problem.

## Stage 4: Scope and Liveness

Track lexical register lifetimes and block boundaries.

## Stage 5: Type Checking

Check operand widths, memory forms, and instruction constraints.

## Stage 6: Control-Flow Lowering

Convert structured `if` and `while` constructs into deterministic low-level branches.

## Stage 7: Assembly Emission

Emit valid x86_64 assembly for supported toolchains.

## Stage 8: ABI Boundaries

Handle calls into external code and system interfaces cleanly.

## Stage 9: Unsafe Idioms

Allow controlled escape hatches for hand-tuned low-level patterns.

## Stage 10: CLI and Tooling

Add a command-line interface, integration tests, and benchmarking.
