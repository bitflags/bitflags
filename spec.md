# Bitflags

`bitflags` generates flags enums with well-defined semantics and ergonimic end-user APIs.

You can use `bitflags` to:

- provide more user-friendly bindings to C APIs where flags may or may not be fully known in advance.
- generate efficient options types with string parsing and formatting support.

You can't use `bitflags` to:

- guarantee only bits corresponding to defined flags will ever be set.

## Definitions

This section formally defines the terminology and semantics of `bitflags`. It's organized so more fundamental concepts are introduced before those that build on them. It may be helpful to start from the bottom of the section and refer back up to concepts defined earlier.

### Bits type

> A type that stores a fixed number bits.

Bits types are typically fixed-width unsigned integers, like `u32`, but may be more exotic.

#### Bit

> A value at a specific location within a bits type that may be set or unset.

### Flag

> A uniquely named set of bits in a bits type.

Names must be unique, but bits are not required to be exclusive to a flag. Bits are not required to be contiguous.

#### Zero-bit flag

> A flag with a set of zero bits.

#### Single-bit flag

> A flag with a set of one bit.

#### Multi-bit flag

> A flag with a set of more than one bit.

### Flags type

> A set of defined flags over a specific bits type.

#### Known bit

> A bit in any defined flag.

#### Unknown bit

> A bit not in any defined flag.

#### Normal

> A flags type that defines no zero-bit flags and all known bits are in at least one corresponding single-bit flag.

A flags type may still be normal if it defines multi-bit flags so long as each of its bits is also in a defined single-bit flag.

In cases where flags are defined by an external source, a flags type can define a single flag with all bits set. This flags type is not normal, but guarantees no bits will ever be truncated.

### Flags value

> An instance of a flags type using its bits type for storage.

#### Empty

> Whether all bits in a flags value are unset.

If any known or unknown bits are set then the flags value is considered not empty.

#### All

> Whether all defined flags are contained in a flags value.

Unknown bits don't affect whether a flags value is all. It's not a strict equality condition like empty.

#### Contains

> Whether all set bits in a source flags value are also set in a target flags value.

If the tatget is empty then the source will always contain it. A flag is contained in a flags value if all bits in the flag are set in the flags value.

#### Intersects

> Whether any set bits in a source flags value are also set in a target flags value.

An empty flags value never intersects any other flags value. A flag intersects a flags value if any bits in the flag are set in the flags value.

#### Normalized

> Whether all set bits in a flags value are known bits and all defined flags that intersect are also contained.

A consequence of zero-bit flags always being contained but never intersected means a flags type that defines one can never be normalized.

### Operations

#### Truncate

> Unset all unknown bits in a flags value.

If the flags type is normal then the result is normalized. If the flags type is not normal then truncating doesn't guarantee a normalized result; only one with no unknown bits set. This allows truncation to be implemented efficiently.

#### Union

> The bitwise or (`|`) of the bits in two flags values, truncating the result.

#### Intersection

> The bitwise and (`&`) of the bits in two flags values, truncating the result.

#### Symmetric difference

> The bitwise exclusive-or (`^`) of the bits in two flags values, truncating the result. 

#### Complement

> The bitwise negation (`!`) of the bits in a flags value, truncating the result.

#### Difference

> The intersection of a source flags value with the negation of a target flags value (`&!`), truncating the result.

### Formatting

Flags values can be formatted and parsed using the following *whitespace-insensitive*, *case-sensitive* grammar:

- _Flags:_ (_Flag_)`|`*
- _Flag:_ _Name_ | _HexNumber_
- _Name:_ The name of any defined flag
- _HexNumber_: `0x`([0-9a-fA-F])*

If a source flags value is formatted and then parsed the result will exactly reproduce the source.

## Implementation

The specification is implemented through the `Flags` trait. An implementor of the `Flags` trait is a flags type. An instance of the implementor is a flags value.

### `type Bits`

```rust
type Bits: Bits;
```

The bits type used.

### `const FLAGS`

```rust
const FLAGS: &'static [Flag<Self>];
```

Defines the set of flags.

### `fn bits`

```rust
fn bits(&self) -> Self::Bits;
```

Get the value of the underlying bits type.

The result won't be truncated.

### `fn from_bits_truncate`

```rust
fn from_bits_truncate(bits: Self::Bits) -> Self;
```

Get a flags value with only the known bits in `bits` set.

### `fn from_bits`

```rust
fn from_bits(bits: Self::Bits) -> Option<Self>;
```

Get a flags value with only the known bits in `bits` set.

If the result is non-empty this function will return `Some`, otherwise it will return `None`.

### `fn from_bits_retain`

```rust
fn from_bits_retain(bits: Self::Bits) -> Self;
```

Get a flags value with exactly the bits in `bits` set.

Prefer `from_bits_truncate` where possible; this method is necessary as a building block, but not intended for end-users. If `bits` has any unknown bits set then they'll be truncated by any operations on the returned flags type.

### `fn from_name`

```rust
fn from_name(name: &str) -> Option<Self>;
```

Get a flags value with the bits for a defined flag with the given name set.

If there is a flag defined with `name` this function will return `Some`, otherwise it will return `None`. Names are case-sensitive.

### `fn empty`

```rust
fn empty() -> Self;
```

Get a flags value with all bits unset.

The returned flags value will satisfy `is_empty`.

### `fn all`

```rust
fn all() -> Self;
```

Get a flags value with all known bits set and all unknown bits unset.

The returned flags value will satisfy `is_all`.

### `fn is_empty`

```rust
fn is_empty(&self) -> bool;
```

Whether all bits in the flags value are unset.

### `fn is_all`

```rust
fn is_all(&self) -> bool;
```

Whether all defined flags are contained in the flags value.

### `fn intersection`

```rust
fn intersection(self, other: Self) -> Self;
```

Calculates the intersection of the bits in `self` and `other`.

The result will be truncated.

### `fn intersects`

```rust
fn intersects(&self, other: Self) -> bool;
```

Whether `self` and `other` intersect.

### `fn contains`

```rust
fn contains(&self, other: Self) -> bool;
```

Whether `self` contains `other`.

### `fn union`

```rust
fn union(self, other: Self) -> Self;
```

Calculates the union of the bits in `self` and `other`.

The result will be truncated.

### `fn insert`

```rust
fn insert(&mut self, other: Self);
```

Assigns the union of the bits in `self` and `other`.

The result will be truncated.

### `fn difference`

```rust
fn difference(self, other: Self) -> Self;
```

Calculates the difference between the bits in `self` and `other`.

The result will be truncated.

### `fn remove`

```rust
fn remove(&mut self, other: Self);
```

Assigns the difference between the bits in `self` and `other`.

The result will be truncated.

### `fn set`

```rust
fn set(&mut self, other: Self, value: bool);
```

Assigns the union of `self` and `other` if `value` is `true`, or the difference between `self` and `other` if `value` is `false`.

The result will be truncated.

### `fn symmetric_difference`

```rust
fn symmetric_difference(self, other: Self) -> Self;
```

Calculates the symmetric difference between the bits in `self` and `other`.

The result will be truncated.

### `fn toggle`

```rust
fn toggle(&mut self, other: Self);
```

Calculates the symmetric difference between the bits in `self` and `other`.

The result will be truncated.

### `fn complement`

```rust
fn complement(self) -> Self;
```

Calculates the complement of the bits in `self`.

The result will be truncated.

### `fn iter`

```rust
fn iter(&self) -> iter::Iter<Self>;
```

Iterate over defined flags contained in `self`.

The result of unioning all yielded flags will exactly reproduce `self`.

Each yielded flags value will correspond to a single flag. Not all flags contained in `self` are guaranteed to be yielded; only enough to exactly reproduce `self`. Overlapping flags may be omitted.

If `self` is not normalized then any remaining bits will be yielded as a final result.

### `fn iter_names`

```rust
fn iter_names(&self) -> iter::IterNames<Self>;
```

Iterate over defined flags and their names contained in `self`.

The result of unioning all yielded flags will lossily normalize `self`.

If `self` is normalized then the result of unioning all yielded flags will exactly reproduce `self`. If `self` is not normalized then any remaining bits will not be yielded. Not all flags contained in `self` are guaranteed to be yielded; only enough to lossily normalize `self`.
