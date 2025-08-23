# Reliq
## A Numeric & Data Structure Toolkit designed for Portability.
A lightweight, `no_std` compatible library providing fixed-capacity arrays, UTF-8 strings, fixed-precision numeric types, and a generic numeric trait hierarchy. It is designed for portability where heap allocation may be undesirable, and predictablee arithmetic behaviour is required.

### !!! This crate is not stable yet. !!!

#### Fixed-Capacity Arrays
* `Array<const A: usize, B> -- stack-allocated, compile-time sized array.
* Supports:
    * `push`, `pop`, `insert`, `remove`.
    * `swap_insert` / `swap_remove` for unordered collections.
    * Conversion to slices `as_slice`, `as_mut_slice`, and iteration.
```rust
let mut arr: Array<4, u8> = Array::default();
arr.push(1).unwrap();
arr.push(2).unwrap();
arr.swap_insert(0, 5).unwrap();
assert_eq!(arr.len(), 3);
```


#### UTF-8
* `Utf8<const A: usize>` -- fixed-capacity Utf8 string.
* Supports safe `push`/`pop` of `char`, casting between capacities, and encoding/decoding from byte slices.
* Implements `Display`, `Debug`, `PartialEq`, `Eq`, `Ord`, and `PartialOrd`.
```rust
let mut s: Utf8<16> = Utf8::zero();
s.push('H').unwrap();
s.push('i').unwrap();
assert_eq!(s.as_str(), "Hi");
```

#### Fixed-Precision Rational / Decimal Type
```rust
fn op() -> q::Result<q::Q2<u32>> {
    
}
```

#### Numeric Trait Hierarchy

#### Example



```rust
let mut value = 0;
let result = require(&mut value, |v| {
    *v += 1;
    Err("rollback")
});
assert_eq!(value, 0);
```



#### License
Apache-2.0


