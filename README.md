# misri

Fast NJU IRsim written in rust, with slight modifications in instruction usage.

Up to 70x performance gains compared to the obsolete python version.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `python ../simulator/irsim.py opt-out.ir > mine.log` | 6.296 ± 0.232 | 5.943 | 6.614 | 71.80 ± 10.64 |
| `./misri -f opt-out.ir > std.log` | 0.088 ± 0.013 | 0.070 | 0.101 | 1.00 |

# Build & Test & Run

```bash
cargo build --release
cargo test
./misri --help
```

# TODO

- [ ] A simple debugger
- [ ] JIT?

# IR spec

This section will briefly introduce IR instructions that differ with the
reference manual and the reasons for modifications.

A valid program consists of a list of functions. Each function owns a list of 
instructions as its body. Functions with identical names are not allowd.

`FUNCTION <name> :`

## Variables

The reference manual requires that all variables shouldn't have the same names,
while misri allows different variables from different functions to have 
identical names.

## Arithmetic

### ASSIGN

`<x> := <y>`

Assign the value of `<y>` to `<x>`.

### BINARY

`<x> := <y> <binOp> <z>`

Calculate the value of `<y> <binOp> <z>` according to `<binOp>`, and stores the
result to `<x>`.

Note

1. The behavior of a division-by-zero scenario is undefined.
2. Integer divisions always round towards zero.

## Memory

### DEC

`<x> := DEC <size>`

DEC instructions are used to allocate on-stack memories, especially for arrays
and structs.

The reference manual implies that `<x>` will hold the value of the first 
four bytes allocated by this instruction. For example, executing `x := DEC 8` 
results in `{{x = 0}}`.

This is a bit of annoying since we expect the allocation to yield a pointer to
the first address of that contiguous space. Thus misri changes this behavior.
DEC will return the **address** of the allocated memories, with the hope of
simplifying subsequent procedures to handle memory accesses.

An alternative choice to achieve this could be:
```
tmp := DEC 114  // tmp holds the first 4 bytes of 114 allocated bytes 
x := &tmp       // x now points to the starting address of the 114 bytes
```

### DEREF

`<x> := &<y>`

DEREF instructions takes the address of `<y>`, and stores it to `<x>`.

The reference manual is quiet about taking the addresses of regular variables
(i.e. variables that are not parts of memories allocated by `DEC`).

misri actually ignores dereferencing, and will perform a normal assignment.
If `<y>` is already a pointer, `<x> := &<y>` will be equivalent to `<x> := <y>`.
