# Language Spec

This is the current working shape of the VEND language.

## Core Structure

```vend
fn main(argc, argv) {
    alias ptr: reg64 = RAX

    if ptr > 0 {
        mov ptr, [RBP + R8 * 4 + 16]
    }
}
```

## Main Ideas

- `fn` introduces a function.
- `{ ... }` creates a scope block.
- `alias` binds a name to a physical register with an explicit width.
- instructions use a mnemonic and one or more operands.
- `if` and `while` are structured control-flow nodes.

## Registers

VEND models physical registers explicitly, including common x86_64 general-purpose registers and their sub-registers.

Register widths are represented separately as:

- `reg64`
- `reg32`
- `reg16`
- `reg8`

## Memory Addressing

The language supports explicit bracketed memory forms like:

```vend
[base_reg + index_reg * scale + disp]
```

That keeps pointer math visible instead of hiding it inside ad hoc syntax.
