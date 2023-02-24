# autopad

Adds the `autopad!` proc macro, allowing for structs to be defined with fields at specified offsets.

```Rust
autopad!(

#[repr(C)] // Needed to preserve field orders
struct WithPadding {
    root: u8, // At offset 0x0
    0x100 => partway_in: u32,
    after_offset: u32, // At 0x104
    0x200 => final_field: u8,
}

);
```

The macro will fill the struct with appropriate padding arrays such that each field is properly offset in the result.

The intended use case for this is for structs that represent MMIO devices with sparsely allocated or large gaps between registers.

## Limitations

- Has no effect on structs with `repr(Rust)`
  - Rust can rearrange struct field order at-will, thus there is no guarantee that fields will be correctly offset
  - Must specify `repr(C)` or another repr that maintains ordering
- Padded structs cannot be cleanly initialized
  - The padded arrays must be initialized as well
  - This is not ideal, as it exposes macro internals to the end user, but they probably aren't initializing such structs anyway
- Padded structs cannot be neatly debug printed
  - As above, the padding will be printed as well
  - Can workaround with display potentially, but you probably aren't printing sets of MMIO registers to begin with
- Each invocation of `pad_struct!` will only accept one struct definition

Some of these are fixable, PRs are welcome.
