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

let distance = Inches * 1.0;
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

## Custom Units and Measures

Advanced users may want to add custom units to a measure, or entirely new measures.


### Custom measures

Use the `measure!` macro to create new measures. If you need more example usages, this is the macro used internally by Shrewnit to create all measure and unit types.

```rust

measure!(
    /// Your custom measure type
    pub MyCustomMeasure {
        // Optional. If your measure does not have a standard SI unit, dont include this. 
        si: MyStandardSiUnit,
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
        Self / SomeOtherMeasure => ACompletelyDifferentMeasure in SomeUnit,
    }
);
```

This will create the measure type, the unit types, and any necessary implementations.

### Custom Units

Custom units for existing measures can be created by manually implementing the `UnifOf` trait for a type.

```rust
struct MyCustomUnitOfDistance;
impl UnitOf<Distance> for HalfInches {
    fn to_canonical(converted: Scalar) -> Scalar {
        converted / 2.0
    }
    fn from_canonical(canonical: Scalar) -> Scalar {
        canonical * 2.0
    }
}
```

You can also use the `simple_unit!` macro in order to streamline simple conversions like this.

```rust
shrewnit::simple_unit!(
    pub MyCustomUnitOfDistance of measure Distance = per 2.0 canonical
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
impl UnitOf<Distance> for HalfInches {
    fn to_canonical(converted: Scalar) -> Scalar {
        Inches::to_canonical(converted) * 2.0
    }
    fn from_canonical(canonical: Scalar) -> Scalar {
        Inches::from_canonical(canonical * 2.0)
    }
}
```

# FAQ

> Where does the name come from?

The name is inspired by the etrsucan shrew, the worlds smallest mammal.

> Does this library have any dependencies?

Yes, this library depends on one crate: `num-traits`.
Despite this, Shrewnit is 100% Rust, `no_std`, libm, and alloc free!