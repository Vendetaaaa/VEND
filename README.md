# VEND

VEND is a register-aliased assembly language that tries to make low-level code easier to read, safer to change, and less annoying to debug.

It still speaks in hardware terms. There is no runtime, no garbage collector, and no magic layer hiding what the CPU is doing. The difference is that VEND makes the compiler keep track of the messy parts for you: register scope, sub-register overlap, operand widths, and structured control flow.

## Why This Exists

Assembly is powerful, but it puts a lot of bookkeeping on the programmer. Once a project grows beyond a few hand-written routines, that bookkeeping starts turning into bugs.

VEND is meant to reduce the most common failure points without giving up direct control of the machine:

- a register gets reused too early and silently destroys state
- `rax`, `eax`, `ax`, and `al` overlap in ways that are easy to forget
- an operand is the wrong width and gets truncated or zero-extended in a way you did not mean
- `cmp` / `jmp` chains become hard to follow after a while
- pointer arithmetic and memory operands stop being obvious to the next person who reads the code

## What VEND Solves

The basic idea is simple: keep assembly-level precision, but make the compiler enforce the parts humans are bad at tracking manually.

That means VEND is built to catch things like:

- register clobbering through lexical alias scopes
- sub-register conflicts through explicit physical register tracking
- width mismatches through a strict `reg64` / `reg32` / `reg16` / `reg8` model
- control-flow mistakes through structured `if` and `while` forms
- ambiguous memory operands through explicit addressing forms

## How People Will Use It

The long-term shape of the tool is simple:

- write VEND source files
- run the compiler or checker from the command line
- get either a compiled binary or a precise diagnostic
- use the output in real systems code, not just in a toy demo

That is why the repository is being built with documentation, tests, and stage-by-stage implementation instead of a single giant code drop.

## What Is In This Repo Right Now

This repository currently holds the stage 1 language model:

- AST types for programs, functions, blocks, instructions, operands, and control flow
- the full x86_64-style physical register set used by the language model
- register width types for `reg64`, `reg32`, `reg16`, and `reg8`
- memory-address modeling for bracketed addressing expressions
- a grammar reference string for the language surface
- tests that build nested ASTs directly in memory

## Example

```VEND
fn main(argc: reg32, argv: reg64) {
    alias ptr: reg64 = RAX

    if (ptr > 0) {
        mov ptr, qword ptr [RBP + R8 * 4 + 16]
    } else {
        while (ptr > 0) {
            add ptr, 8
        }
    }
}
```

## Notes

- This is a Rust prototype for the VEND language.
- The README describes the intended direction, not a finished compiler.
- The language is deliberately strict: it should fail fast instead of hiding bugs with silent spills or truncation.

## License

This project uses the MIT License. See [LICENSE](LICENSE).
