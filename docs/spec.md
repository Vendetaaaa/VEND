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
- Statements may be separated by a newline or `;`.
- `mov RAX RBX` and `mov RAX, RBX` are equivalent.

## Registers

VEND models physical registers explicitly, including common x86_64 general-purpose registers and their sub-registers:

- `reg64`
- `reg32`
- `reg16`
- `reg8`

The physical register model uses these bit ranges relative to each 64-bit root:

- `reg64` registers cover bits `0..64`.
- `reg32` registers cover bits `0..32`.
- `reg16` registers cover bits `0..16`.
- Low 8-bit registers cover bits `0..8`; `AH`, `BH`, `CH`, and `DH` cover bits `8..16`.

## Memory Addressing

The language supports explicit bracketed memory forms like:

```vend
[base_reg + index_reg * scale + disp]
```

The base and index are registers or aliases. The base and index are optional;
the index has an unsigned scale, and the displacement is a signed integer.

That keeps pointer math visible
