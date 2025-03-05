//! 100% stable and `no_std` Rust units library with support for custom units and dimensions.
//!
//! # Note
//!
//! Shrewnit deviates from SI in one regard: angle is a base dimension.
//! This means that the units of torque are not Nm, they are Nm/rad or J/rad.
//!
//! # Usage
//!
//! Shrewnit is a type-per-dimension units library, meaning every dimension gets its own type.
//! Each unit has its own type as well, but they are only used for conversions and initialization.
//!
//! ## Creating Quantities
//!
//! Quantities can be created in two ways: multiplication and `ScalarExt`.
//! Multiplication is techinically more correct (quantities are defined as the product of a scalar and a unit),
//! but some may find `ScalarExt` easier to read.
//!
//! Multiplication with unit type:
//!
//! ```rust
//! # use shrewnit::Inches;
//! let distance = 1.0 * Inches;
//!
//! let distance = Inches * 1.0;
//! ```
//!
//! `ScalarExt`:
//!
//! ```rust
//! # use shrewnit::ScalarExt;
//! let distance = 1.0.inches();
//! ```
//!
//! ## Unit Math
//!
//! Quantities can be multiplied and divided with un-united scalars, but not added or subtracted.
//!
//! ```rust
//! # use shrewnit::Seconds;
//! let mut quantity = 1.0 * Seconds * 2.0;
//! quantity /= 4.0;
//! ```
//!
//! Additional operations are supported depending on the dimension of the quantity.
//! For example, multiplying a `LinearVelocity` with a `Time` will result in a `Distance`.
//!
//! ```rust
//! # use shrewnit::{Seconds, MilesPerHour};
//! let time = 5.0 * Seconds;
//! let change_in_velocity = 60.0 * MilesPerHour;
//!
//! let acceleration = change_in_velocity / time;
//! ```
//!
//! ## Accessing the Value
//!
//! To get the value of a quantity, use the `Dimension` trait's `to` function.
//!
//! ```rust
//! use shrewnit::{Dimension, Minutes, Seconds};
//! let time = 5.0f32 * Seconds;
//!
//! println!("{}", time.to::<Minutes>());
//! ```
//!
//! If you prefer your code to read like English, you can use the `to!` macro.
//!
//! ```rust
//! # use shrewnit::{Dimension, Minutes, Seconds, to};
//! let time = 5.0f32 * Seconds;
//!
//! println!("{}", to!(time in Minutes));
//! ```
//!
//! ## Custom Units and Measures
//!
//! Advanced users may want to add custom units to a dimension, or entirely new measures.
//!
//! ### Custom measures
//!
//! Use the `dimension!` macro to create new measures.
//! If you need more example usages,
//! this is the macro used internally by Shrewnit to create all dimension and unit types.
//!
//! ```ignore
//! shrewnit::dimension!(
//!     /// Your custom dimension type
//!     pub MyCustomMeasure {
//!         // Shrewnit uses standard SI units as canonical units. This isn't required. Do whatever you feel like.
//!         canonical: MyStandardSiUnit,
//!
//!         // Conversion can be read as "one MyStandardSiUnit per canonical unit"
//!         MyStandardSiUnit: 1.0 per canonical,
//!         // Conversion can be read as "two MyHalfUnits per canonical unit"
//!         MyHalfUnit: 2.0 per canonical,
//!         // Conversion can be read as "one MyDoubleUnits per two canonical units"
//!         MyDoubleUnit: per 2.0 canonical,
//!     } where {
//!         // Optional operations block.
//!         // Self </ or *> <other or same dimension type> => <output dimension type> in <output units>
//!         Self / SomeOtherMeasure => ACompletelyDifferentMeasure in SomeUnit,
//!     }
//! );
//! ```
//!
//! This will create the dimension type, the unit types, and any necessary implementations.
//!
//! ### Custom Units
//!
//! Custom units for existing measures can be created by manually implementing the `UnifOf` trait for a type.
//!
//! ```rust
//! # use shrewnit::{Scalar, Length, UnitOf};
//! struct MyCustomUnitOfLength;
//! impl<S: Scalar> UnitOf<S, Length<S>> for MyCustomUnitOfLength {
//!     fn to_canonical(converted: S) -> S {
//!         converted / S::from_f64(2.0).unwrap()
//!     }
//!     fn from_canonical(canonical: S) -> S {
//!         canonical * S::from_f64(2.0).unwrap()
//!     }
//! }
//! ```
//!
//! You can also use the `simple_unit!` macro in order to streamline simple conversions like this.
//!
//! ```rust
//! # use shrewnit::Length;
//! shrewnit::simple_unit!(
//!     pub MyCustomUnitOfDistance of dimension Length = per 2.0 canonical
//! );
//! ```
//!
//! The conversions will be in terms of the dimension's canonical unit.
//! The canonical unit for all Shrewnit measures are the standard SI unit.
//! If you do not know what this is, go to the definition of the dimension.
//! The canonical unit is the one marked with `canonical: <unit>`.
//!
//! ```ignore
//! dimension!(
//!     pub Torque {
//!         si: NewtonMeters,
//!         // This is our canonical unit.
//!         canonical: NewtonMeters,
//! ...
//! ```
//!
//! You can also base your unit conversions off of existing units. Unfortunately, you cannot do this using `simple_unit!`.
//!
//! ```rust
//! # use shrewnit::{Scalar, Length, UnitOf, Inches};
//! struct HalfInches;
//! impl<S: Scalar> UnitOf<S, Length<S>> for HalfInches {
//!     fn to_canonical(converted: S) -> S {
//!         Inches::to_canonical(converted) * S::from_f64(2.0).unwrap()
//!     }
//!     fn from_canonical(canonical: S) -> S {
//!         Inches::from_canonical(canonical) * S::from_f64(2.0).unwrap()
//!     }
//! }
//! ```
//!
//! # FAQ
//!
//! > Where does the name come from?
//!
//! The name is inspired by the etrsucan shrew, the worlds smallest mammal.
//!
//! > What does this library depend on?
//!
//! Shrewnit depends on one crate: `num-traits`.
//! Despite this, Shrewnit is 100% Rust, `no_std`, libm, and alloc free!

#![no_std]

pub mod dimensions;
use core::ops::{Add, Div, Mul, Sub};

pub use dimensions::*;
use num_traits::FromPrimitive;

/// A set of requirements for a scalar type to be used in measures.
///
/// This trait is automatically implemented for any type that implements `FromPrimitive`, `Clone`, and the basic arithmetic operations.
pub trait Scalar:
    FromPrimitive
    + Clone
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
{
}
impl<
        T: FromPrimitive
            + Clone
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + Add<T, Output = T>
            + Sub<T, Output = T>,
    > Scalar for T
{
}

/// A trait implemented by all physical quantities.
pub trait Dimension<S: Scalar = f64> {
    type CanonicalUnit: UnitOf<S, Self>;

    /// Converts the dimension to the given unit.
    #[inline]
    fn to<U: UnitOf<S, Self>>(&self) -> S
    where
        Self: Sized,
    {
        U::from_canonical(self.canonical())
    }

    /// Creates a new dimension from the given scalar and unit.
    ///
    /// # Note
    ///
    /// Usage of this function directly is discouraged. Instead, use multiplication or the `ScalarExt` trait.
    ///
    /// ```
    /// # use shrewnit::{Meters, ScalarExt};
    ///
    /// let quantity = 30.0f32 * Meters;
    /// let quantity = 30.0f32.meters();
    /// ```
    #[inline]
    fn from_scalar<U: UnitOf<S, Self>>(value: S) -> Self
    where
        Self: Sized,
    {
        Self::from_canonical(U::to_canonical(value))
    }

    /// Returns the canonical representation of the dimension.
    fn canonical(&self) -> S;
    /// Creates a new dimension from the canonical representation.
    fn from_canonical(value: S) -> Self;
}

#[macro_export]
macro_rules! to {
    ($dimension:ident in $unit:ty) => {
        $dimension.to::<$unit>()
    };
}

pub trait UnitOf<S: Scalar, M: Dimension<S> + ?Sized> {
    /// Converts a scalar value from the canonical unit to unit of `Self`.
    fn from_canonical(canonical: S) -> S;
    /// Converts a scalar value from the unit of `Self` to the canonical unit.
    fn to_canonical(converted: S) -> S;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __measure_conversions {
    {} => {};
    {$self:ty,} => {};
    ($self:ident, Self * $rhs:ident => $output:ident in $output_unit:ty, $($rest:tt)*) => {
        impl<S: $crate::Scalar> core::ops::Mul<$rhs<S>> for $self<S> {
            type Output = $output<S>;
            fn mul(self, rhs: $rhs<S>) -> Self::Output {
                use $crate::Dimension;
                $output::from_scalar::<$output_unit>(self.canonical() * rhs.canonical())
            }
        }

        $crate::__measure_conversions!($self, $($rest)*);
    };
    ($self:ident, Self / $rhs:ident => $output:ident in $output_unit:ty, $($rest:tt)*) => {
        impl<S: $crate::Scalar> core::ops::Div<$rhs<S>> for $self<S> {
            type Output = $output<S>;
            fn div(self, rhs: $rhs<S>) -> Self::Output {
                use $crate::Dimension;
                $output::from_scalar::<$output_unit>(self.canonical() / rhs.canonical())
            }
        }

        $crate::__measure_conversions!($self, $($rest)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __unit_mult_imp {
    ($unit:ident, $dimension:ident, $($rhs:ident),*) => {
        $(
            impl core::ops::Mul<$unit> for $rhs {
                type Output = $dimension<$rhs>;
                fn mul(self, _rhs: $unit) -> $dimension<$rhs> {
                    use $crate::Dimension;
                    $dimension::from_scalar::<$unit>(self)
                }
            }
        )*
    };
}

/// A macro for creating a new unit type.
///
/// This macro creates a new unit type and implements multiplication with scalars on it.
///
/// # Note
///
/// you will have to implement the `UnitOf` trait for the new unit type for this to compile.
///
/// # Examples
///
/// ```
/// use shrewnit::{Length, UnitOf};
///
/// shrewnit::unit_type!(
///     /// A unit of length.
///     pub Feet of dimension Length
/// );
///
/// impl<S: shrewnit::Scalar> UnitOf<S, Length<S>> for Feet {
///     fn from_canonical(canonical: S) -> S {
///         canonical / S::from_f64(3.28084).unwrap()
///     }
///     fn to_canonical(converted: S) -> S {
///         converted * S::from_f64(3.28084).unwrap()
///     }
/// }
/// ```
#[macro_export]
macro_rules! unit_type {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of dimension $dimension:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        $vis struct $unit;

        impl<S: $crate::Scalar> core::ops::Mul<S> for $unit {
            type Output = $dimension<S>;
            fn mul(self, rhs: S) -> $dimension<S> {
                use $crate::Dimension;
                $dimension::from_scalar::<$unit>(rhs)
            }
        }

        impl $unit {
            #[inline]
            pub fn from_scalar<S: $crate::Scalar>(value: S) -> $dimension<S> {
                use $crate::Dimension;
                $dimension::from_scalar::<Self>(value)
            }
        }

        $crate::__unit_mult_imp!(
            $unit,
            $dimension,
            f64,
            f32,
            i8,
            i16,
            i32,
            i64,
            i128,
            isize,
            u8,
            u16,
            u32,
            u64,
            u128,
            usize
        );
    };
}

/// A macro for creating a new unit type with simple conversions. Used internally by [`dimension!`](dimension)
///
/// Conversions are implemented by multiplying or dividing by a scalar value.
///
/// # Examples
///
/// ```
/// use shrewnit::{Length};
///
/// shrewnit::simple_unit!(
///     /// A unit of length.
///     pub Feet of dimension Length = 3.28084 per canonical
/// );
/// ```
#[macro_export]
macro_rules! simple_unit {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of dimension $dimension:ident = $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?
    ) => {
        $crate::unit_type!(
            $(#[$meta])*
            $vis $unit of dimension $dimension
        );

        $(
            impl<S: $crate::Scalar> $crate::UnitOf<S, $dimension<S>> for $unit {
                #[inline]
                fn from_canonical(canonical: S) -> S {
                    canonical * S::from_f64($rhsper).unwrap()
                }
                #[inline]
                fn to_canonical(converted: S) -> S {
                    converted / S::from_f64($rhsper).unwrap()
                }
            }
        )?
        $(
            impl<S: $crate::Scalar> $crate::UnitOf<S, $dimension<S>> for $unit {
                #[inline]
                fn from_canonical(canonical: S) -> S {
                    canonical /  S::from_f64($lhsper).unwrap()
                }
                #[inline]
                fn to_canonical(converted: S) -> S {
                    converted *  S::from_f64($lhsper).unwrap()
                }
            }
        )?
    };
}

/// A macro for creating a new dimension type and any simple associated unit types.
/// Associated unit types are parsed using similar syntax to the [`simple_unit!`] macro.
///
/// # Examples
///
/// This macro is used extensively in the library to create all dimension and unit types.
/// Look in the dimensions module source for more examples.
///
/// ```
/// shrewnit::dimension!(
///     /// A dimension of some kind.
///     pub MyCustomDimension {
///         canonical: MyStandardUnit,
///         
///         MyStandardUnit: 1.0 per canonical,
///
///         MyHalfUnit: 2.0 per canonical,
///         MyDoubleUnit: per 2.0 canonical,
///     } where {
///         // Optional conversions block.
///         // Self </ or *> <other or same dimension type> => <output dimension type> in <output units>
///     }
/// );
/// ```
#[macro_export]
macro_rules! dimension {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            canonical: $canonical_unit:ident,

            $(
                $(#[$unit_meta:meta])*
                $unit:ident: $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?,
            )+
        } $(where {
            $($converts:tt)*
        })?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        $vis struct $name<S: $crate::Scalar = f64>(S);

        impl<S: $crate::Scalar> $crate::Dimension<S> for $name<S> {
            type CanonicalUnit = $canonical_unit;

            #[inline]
            fn canonical(&self) -> S {
                self.0.clone()
            }
            #[inline]
            fn from_canonical(value: S) -> Self {
                Self(value)
            }
        }

        impl<S: $crate::Scalar + core::fmt::Debug> core::fmt::Debug for $name<S> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?} {})", stringify!($name), self.0, stringify!($canonical_unit))
            }
        }

        impl<S: $crate::Scalar> core::ops::Mul<S> for $name<S> {
            type Output = $name<S>;
            fn mul(self, rhs: S) -> $name<S> {
                $name(self.0 * rhs)
            }
        }
        impl<S: $crate::Scalar> core::ops::MulAssign<S> for $name<S> {
            fn mul_assign(&mut self, rhs: S) {
                self.0 = self.0.clone() * rhs;
            }
        }

        impl<S: $crate::Scalar> core::ops::Div<S> for $name<S> {
            type Output = $name<S>;
            fn div(self, rhs: S) -> $name<S> {
                $name(self.0 / rhs)
            }
        }
        impl<S: $crate::Scalar> core::ops::DivAssign<S> for $name<S> {
            fn div_assign(&mut self, rhs: S) {
                self.0 = self.0.clone() / rhs;
            }
        }

        impl<S: $crate::Scalar> core::ops::Add<$name<S>> for $name<S> {
            type Output = $name<S>;
            fn add(self, rhs: $name<S>) -> $name<S> {
                $name(self.0 + rhs.0)
            }
        }
        impl<S: $crate::Scalar> core::ops::AddAssign<$name<S>> for $name<S> {
            fn add_assign(&mut self, rhs: $name<S>) {
                self.0 = self.0.clone() + rhs.0;
            }
        }
        impl<S: $crate::Scalar> core::ops::Sub<$name<S>> for $name<S> {
            type Output = $name<S>;
            fn sub(self, rhs: $name<S>) -> $name<S> {
                $name(self.0 - rhs.0)
            }
        }
        impl<S: $crate::Scalar> core::ops::SubAssign<$name<S>> for $name<S> {
            fn sub_assign(&mut self, rhs: $name<S>) {
                self.0 = self.0.clone() - rhs.0;
            }
        }

        $(
            $crate::simple_unit!(
                $(#[$unit_meta])*
                $vis $unit of dimension $name = $($rhsper per canonical)? $(per $lhsper canonical)?
            );
        )*

        $(
            $crate::__measure_conversions!($name, $($converts)*);
        )?
    };
}

/// A convenient way to implement extension traits for scalars that allows for quantity construction.
///
/// # Examples
///
/// ```
/// use shrewnit::{Length, Millimeters, Centimeters, Meters};
///
/// shrewnit::scalar_extension_trait!(
///     pub trait ScalarExt {
///         Length {
///             millimeters => Millimeters,
///             centimeters => Centimeters,
///             meters => Meters
///             // ...
///         }
///         // ...
///     }
/// );
/// ```
#[macro_export]
macro_rules! scalar_extension_trait {
    (
        $(#[$meta:meta])*
        $vis:vis trait $name:ident {
            $(
                $dimension:ident {
                    $(
                        $func_name:ident => $unit:ident
                    ),*
                }
            ),*
        }
    ) => {
        $(#[$meta])*
        $vis trait $name<S: $crate::Scalar> {
            $(
                $(
                    fn $func_name(self) -> $dimension<S>;
                )*
            )*
        }
        impl<S: $crate::Scalar> $name<S> for S {
            $(
                $(
                    #[inline]
                    fn $func_name(self) -> $dimension<S> {
                        $unit::from_scalar(self)
                    }
                )*
            )*
        }
    };
}

scalar_extension_trait!(
    /// An extension trait for scalars that allows for quantity construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use shrewnit::ScalarExt;
    ///
    /// let quantity = 100.0.millimeters();
    /// ```
    pub trait ScalarExt {
        Length {
            millimeters => Millimeters,
            centimeters => Centimeters,
            meters => Meters,
            kilometers => Kilometers,

            inches => Inches,
            feet => Feet,
            miles => Miles,
            yards => Yards,
            nautical_miles => NauticalMiles
        },

        Area {
            square_millimeters => SquareMillimeters,
            square_centimeters => SquareCentimeters,
            square_meters => SquareMeters,
            square_kilometers => SquareKilometers,

            square_inches => SquareInches,
            square_feet => SquareFeet,
            square_yards => SquareYards,
            acres => Acres
        },

        Volume {
            milliliters => Milliliters,
            liters => Liters,

            cubic_millimeters => CubicMillimeters,
            cubic_centimeters => CubicCentimeters,
            cubic_meters => CubicMeters,
            cubic_kilometers => CubicKilometers,

            cubic_inches => CubicInches,
            cubic_feet => CubicFeet,
            cubic_yards => CubicYards,

            fluid_ounces => FluidOunces,
            pints => Pints,
            quarts => Quarts,
            gallons => Gallons
        },

        Time {
            microseconds => Microseconds,
            milliseconds => Milliseconds,
            seconds =>  Seconds,
            minutes => Minutes,
            hours => Hours,
            days => Days,
            weeks => Weeks,
            years => Years
        },

        LinearVelocity {
            meters_per_second => MetersPerSecond,
            kilometers_per_second => KilometersPerSecond,
            kilometers_per_hour => KilometersPerHour,
            feet_per_second => FeetPerSecond,
            miles_per_hour => MilesPerHour
        },

        LinearAcceleration {
            meters_per_second_squared => MetersPerSecondSquared,
            feet_per_second_squared => FeetPerSecondSquared
        },

        Angle {
            radians => Radians,
            rotations => Rotations,
            degrees => Degrees,
            gradians => Gradians
        },

        AngularVelocity {
            radians_per_second => RadiansPerSecond,
            rotations_per_second => RotationsPerSecond,
            rotations_per_minute => RotationsPerMinute,
            degrees_per_second => DegreesPerSecond
        },

        Mass {
            kilograms => Kilograms,
            grams => Grams,
            milligrams => Milligrams,
            micrograms => Micrograms,

            pounds => Pounds,
            ounces => Ounces,
            stones => Stones,

            metric_tons => MetricTons,
            short_tons => ShortTons,
            long_tons => LongTons
        },

        Force {
            newtons => Newtons,
            pounds_force => PoundsForce,
            dynes => Dynes
        },

        Pressure {
            pascals => Pascals,
            psi => Psi,
            atmospheres => Atmospheres,
            bars => Bars
        },

        Torque {
            newton_meters_per_radian => NewtonMetersPerRadian,
            newton_meters_per_degree => NewtonMetersPerDegree,

            pound_feet_per_radian => PoundFeetPerRadian,
            pound_feet_per_degree => PoundFeetPerDegree,

            dyne_centimeters_per_radians => DyneCentimetersPerRadians
        },

        Energy {
            joules => Joules,
            calories => Calories,
            kilocalories => Kilocalories,
            ergs => Ergs,
            watt_hours => WattHours
        },

        Power {
            watts => Watts,
            horsepower => Horsepower,
            ergs_per_second => ErgsPerSecond,
            foot_pounds_per_minute => FootPoundsPerMinute
        },

        Voltage {
            volts => Volts,
            millivolts => Millivolts,
            kilovolts => Kilovolts
        },

        Current {
            amperes => Amperes,
            milliamperes => Milliamperes,
            kiloamperes => Kiloamperes
        },

        Temperature {
            kelvin => Kelvin,
            celsius => Celsius,
            fahrenheit => Fahrenheit
        }
    }
);
