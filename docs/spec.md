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
- `unsafe { ... }` opts an ordinary lexical block into hand-tuned machine
  operations. It is not a second language or an escape from parsing and
  emission. It affects only the following operand and condition intent checks:
  - matching register widths for `mov`, `add`, `sub`, `xor`, `cmp`, and `test`;
  - matching condition widths;
  - 64-bit width for memory base and index registers; and
  - the valid memory-index scales `1`, `2`, `4`, and `8`.

  The block keeps normal syntax validation, alias declaration-width validation,
  alias conflict/visibility/liveness validation, lowering, and ABI call-shape
  validation. It does not relax alias lookup: aliases declared outside remain
  visible according to lexical scope, aliases declared inside end at that
  block's scope, and unresolved aliases are still diagnostics. For example,
  `unsafe { mov RAX, ECX }` is accepted, because the 64-bit/32-bit pairing is
  intentionally machine-level; `mov missing, RAX` is not.
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
