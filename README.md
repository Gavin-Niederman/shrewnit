# Shrewnit

> Simple, stable, extendible units.

Shrewnit is a 100% stable and `no_std` Rust units library.
Shrewnit has support for adding custom unit types.

# Usage

Shrewnit is a type-per-dimension units library, meaning every dimension gets its own type.
Each unit has its own type as well, but they are only used for conversions and initialization of quantities.

## Creating Quantities

Quantities can be created in two ways: multiplication and `ScalarExt`.
Multiplication is techinically more correct (quantities are defined as the product of a scalar and a unit),
but some may find `ScalarExt` easier to read.

Multiplication with unit type:

```rust
let distance = 1.0 * Inches;

let distance = Inches * 1.0;
```

`ScalarExt`:

```rust
let distance = 1.0.inches();
```

## Unit Math

Quantities can be multiplied and divided with un-united scalars, but not with added or subtracted.

```rust
let mut quantity = 1.0 * Seconds * 2.0;
quantity /= 4.0;
```

Additional operations are supported depending on the dimension of the quantity.
For example, multiplying a `LinearVelocity` with a `Time` will result in a `Length`.

```rust
let time = 5.0 * Seconds;
let change_in_velocity = 60.0 * MilesPerHour;

let acceleration = change_in_velocity / time;
```

## Accessing the Value

To get the value of a measure, use the `Dimension` trait's `to` function.

```rust
let time = 5.0 * Seconds;

println!("{}", time.to::<Minutes>());
```

## Custom Units and Dimensions

Advanced users may want to add custom units to a measure, or entirely new measures.

### Custom dimensions

Use the `dimension!` macro to create new measures. If you need more example usages, this is the macro used internally by Shrewnit to create all measure and unit types.

```rust

shrewnit::dimension!(
    /// Your custom dimension type
    pub MyCustomDimension {
        // Shrewnit uses standard SI units as canonical units. This isn't required. Do whatever you feel like.
        canonical: MyStandardSiUnit,

        // Conversion can be read as "one MyStandardSiUnit per canonical unit"
        MyStandardSiUnit: 1.0 per canonical,
        // Conversion can be read as "two MyHalfUnits per canonical unit"
        MyHalfUnit: 2.0 per canonical,
        // Conversion can be read as "one MyDoubleUnits per two canonical units"
        MyDoubleUnit: per 2.0 canonical,
    } where {
        // Optional operations block.
        // Self </ or *> <other or same measure type> => <output measure type> in <output units>
        Self / SomeOtherDimension => ACompletelyDifferentDimension in SomeUnit,
    }
);
```

This will create the measure type, the unit types, and any necessary implementations.

### Custom Units

Custom units for existing dimensions can be created by manually implementing the `UnitOf` trait for a type.

```rust
struct MyCustomUnitOfLength;
impl<S: Scalar> UnitOf<S, Length<S>> for MyCustomUnitOfLength {
    fn to_canonical(converted: S) -> S {
        converted / S::from_f64(2.0).unwrap()
    }
    fn from_canonical(canonical: S) -> S {
        canonical * S::from_f64(2.0).unwrap()    }
}
```

You can also use the `simple_unit!` macro in order to streamline simple conversions like this.

```rust
shrewnit::simple_unit!(
    pub MyCustomUnitOfLength of dimension Length = per 2.0 canonical
);
```

The conversions will be in terms of the measure's canonical unit. The canonical unit for all Shrewnit measures are the standard SI unit. If you do not know what this is, go to the definition of the measure. The canonical unit is the one marked with `canonical: <unit>`.

```rust
measure!(
    pub Torque {
        si: NewtonMeters,
        // This is our canonical unit.
        canonical: NewtonMeters,
...
```

You can also base your unit conversions off of existing units. Unfortunately, you cannot do this using `simple_unit!`.

```rust
struct HalfInches;
impl<S: Scalar> UnitOf<S, Length<S>> for HalfInches {
    fn to_canonical(converted: S) -> S {
        Inches::to_canonical(converted) * S::from_f64(2.0).unwrap()
    }
    fn from_canonical(canonical: S) -> S {
        Inches::from_canonical(canonical * S::from_f64(2.0).unwrap())
    }
}
```

# FAQ

> Where does the name come from?

The name is inspired by the etrsucan shrew, the worlds smallest mammal.

> What does this library depend on?

Shrewnit depends on one crate: `num-traits`.
Despite this, Shrewnit is 100% Rust, `no_std`, libm, and alloc free!