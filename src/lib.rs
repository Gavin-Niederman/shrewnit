//! Tiny **stable Rust** unit library built with macros.

#![no_std]

pub mod measures;
use core::ops::{Add, Div, Mul, Sub};

pub use measures::*;
use num_traits::FromPrimitive;

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

pub trait Measure<S: Scalar = f64> {
    type CanonicalUnit: UnitOf<S, Self>;

    #[inline]
    fn to<U: UnitOf<S, Self>>(&self) -> S
    where
        Self: Sized,
    {
        U::from_canonical(self.canonical())
    }

    #[inline]
    fn of<U: UnitOf<S, Self>>(value: S) -> Self
    where
        Self: Sized,
    {
        Self::from_canonical(U::to_canonical(value))
    }

    fn canonical(&self) -> S;
    fn from_canonical(value: S) -> Self;
}
pub trait UnitOf<S: Scalar, M: Measure<S> + ?Sized> {
    fn from_canonical(canonical: S) -> S;
    fn to_canonical(converted: S) -> S;
}

/// Represents the standard SI unit of any measure.
///
/// # Examples
///
/// ```
/// use shrewnit::Measure;
///
/// let velocity = 30.0 * shrewnit::MetersPerSecond;
/// let distance = 100.0 * shrewnit::Meters;
/// let time = 3.0 * shrewnit::Seconds;
///
/// println!("{}", velocity.to::<shrewnit::Si>());
/// println!("{}", distance.to::<shrewnit::Si>());
/// println!("{}", time.to::<shrewnit::Si>());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Si;

#[macro_export]
#[doc(hidden)]
macro_rules! __measure_conversions {
    {} => {};
    {$self:ty,} => {};
    ($self:ident, Self * $rhs:ident => $output:ident in $output_unit:ty, $($rest:tt)*) => {
        impl<S: $crate::Scalar> core::ops::Mul<$rhs<S>> for $self<S> {
            type Output = $output<S>;
            fn mul(self, rhs: $rhs<S>) -> Self::Output {
                use $crate::Measure;
                $output::of::<$output_unit>(self.canonical() * rhs.canonical())
            }
        }

        $crate::__measure_conversions!($self, $($rest)*);
    };
    ($self:ident, Self / $rhs:ident => $output:ident in $output_unit:ty, $($rest:tt)*) => {
        impl<S: $crate::Scalar> core::ops::Div<$rhs<S>> for $self<S> {
            type Output = $output<S>;
            fn div(self, rhs: $rhs<S>) -> Self::Output {
                use $crate::Measure;
                $output::of::<$output_unit>(self.canonical() / rhs.canonical())
            }
        }

        $crate::__measure_conversions!($self, $($rest)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __unit_mult_imp {
    ($unit:ident, $measure:ident, $($rhs:ident),*) => {
        $(
            impl core::ops::Mul<$unit> for $rhs {
                type Output = $measure<$rhs>;
                fn mul(self, _rhs: $unit) -> $measure<$rhs> {
                    use $crate::Measure;
                    $measure::of::<$unit>(self)
                }
            }
        )*
    };
}
#[macro_export]
macro_rules! simple_unit {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of measure $measure:ident = $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        $vis struct $unit;

        impl<S: $crate::Scalar> core::ops::Mul<S> for $unit {
            type Output = $measure<S>;
            fn mul(self, rhs: S) -> $measure<S> {
                use $crate::Measure;
                $measure::of::<$unit>(rhs)
            }
        }

        $crate::__unit_mult_imp!(
            $unit,
            $measure, 
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

        impl $unit {
            #[inline]
            pub fn of<S: $crate::Scalar>(value: S) -> $measure<S> {
                use $crate::Measure;
                $measure::of::<Self>(value)
            }
        }

        $(
            impl<S: $crate::Scalar> $crate::UnitOf<S, $measure<S>> for $unit {
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
            impl<S: $crate::Scalar> $crate::UnitOf<S, $measure<S>> for $unit {
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

#[macro_export]
macro_rules! measure {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            $(si: $si_unit:ident,)?
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
        #[derive(Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name<S: $crate::Scalar = f64>(S);

        impl<S: $crate::Scalar> $crate::Measure<S> for $name<S> {
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
                $vis $unit of measure $name = $($rhsper per canonical)? $(per $lhsper canonical)?
            );
        )*

        $(
            impl<S: $crate::Scalar> $crate::UnitOf<S, $name<S>> for $crate::Si {
                #[inline]
                fn from_canonical(canonical: S) -> S {
                    $si_unit::from_canonical(canonical)
                }
                #[inline]
                fn to_canonical(converted: S) -> S {
                    $si_unit::to_canonical(converted)
                }
            }
        )?

        $(
            $crate::__measure_conversions!($name, $($converts)*);
        )?
    };
}

/// A convenient way to implement extension traits for scalars that allows for measure construction.
#[macro_export]
macro_rules! scalar_extension_trait {
    (
        trait $name:ident {
            $(
                $measure:ident {
                    $(
                        $func_name:ident => $unit:ident
                    ),*
                }
            ),*
        }
    ) => {
        pub trait $name<S: $crate::Scalar> {
            $(
                $(
                    fn $func_name(self) -> $measure<S>;
                )*
            )*
        }
        impl<S: $crate::Scalar> $name<S> for S {
            $(
                $(
                    #[inline]
                    fn $func_name(self) -> $measure<S> {
                        $unit::of(self)
                    }
                )*
            )*
        }
    };
}

scalar_extension_trait!(
    trait ScalarExt {
        Distance {
            millimeters => Millimeters,
            centimeters => Centimeters,
            meters => Meters,
            kilometers => Kilometers,
            inches => Inches,
            feet => Feet
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
            degrees => Degrees
        },

        AngularVelocity {
            radians_per_second => RadiansPerSecond,
            rotations_per_second => RotationsPerSecond,
            rotations_per_minute => RotationsPerMinute,
            degrees_per_second => DegreesPerSecond
        },

        Force {
            newtons => Newtons,
            pounds => Pounds
        },

        Torque {
            newton_meters => NewtonMeters,
            foot_pounds => FootPounds
        }
    }
);
