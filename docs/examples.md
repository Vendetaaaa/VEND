# Examples

## Simple Alias

```vend
fn load_value() {
    alias ptr: reg64 = RAX
    mov ptr, [RBP + 16]
}
```

## Structured Loop

```vend
fn sum() {
    alias count: reg32 = ECX
    alias base: reg64 = RSI

    while count > 0 {
        mov EAX, [base]
        add base, 4
        dec count
    }
}
```

## Conditional Branch

```vend
fn choose(flag) {
    if flag > 0 {
        mov RAX, 1
    } else {
        mov RAX, 0
    }
}
```


## Explicit Unsafe Hand-Tuning

Use `unsafe` only for operations whose deliberate register pairing or memory
address encoding is lower-level than VEND's ordinary intent checks. The block
does not make unknown aliases or malformed external calls valid:

```vend
fn overwrite() {
    alias ptr: reg64 = RAX
    unsafe {
        mov RAX, ECX
        mov ptr, [RCX]
    }
}
```

### Both comma-sep & same-line whitespace-sep operands are accepted.

```vend
fn copy() {
    mov RAX, RBX
    mov RCX RDX; nop
}
```
