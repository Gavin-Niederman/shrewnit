# Shrewnit

> Simple, stable, extendible units.

Shrewnit is a 100% stable and `no_std` Rust units library.
Shrewnit has support for adding custom unit types.

# Usage

Shrewnit is a type-per-measure units library, meaning every measure gets its own type.
Each unit has its own type as well, but they are only used for conversions and initialization.

## Creating Measures

Measures can be created in several ways. All of the following examples are equivalent, but with increasing levels of verbosity. Use whatever what you like most.

Multiplication with unit type:

```rust
let distance = 1.0 * Inches;
```

Unit type method:

```rust
let distance = Inches::of(1.0);
```

Measure type method:

```rust
let distance = Distance::<Inches>::of(1.0)
```

## Unit Math

Measures can be multiplied and divided with un-united scalars, but not with added or subtracted.

```rust
let mut measure = 1.0 * Seconds * 2.0;
measure /= 4.0;
```

Additional operations are supported depending on the measure.
For example, multiplying a `LinearVelocity` with a `Time` will result in a `Distance`.

```rust
let time = 5.0 * Seconds;
let change_in_velocity = 60.0 * MilesPerHour;

let acceleration = change_in_velocity / time;
```

## Accessing the Value

To get the value of a measure, use the `Measure` trait's `to` function.

```rust
let time = 5.0 * Seconds;

println!("{}", time.to::<Minutes>());
```