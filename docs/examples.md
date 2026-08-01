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
