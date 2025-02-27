//! Tiny **stable Rust** unit library built with macros.

#![no_std]

pub mod measures;
pub use measures::*;

pub type Scalar = f64;

mod private {
    pub trait Sealed {}
    pub trait MeasureInternal: Sealed {
        fn canonical(&self) -> super::Scalar;
        fn from_canonical(value: super::Scalar) -> Self;
    }
}

pub trait Measure: private::MeasureInternal {
    fn to<U: UnitOf<Self>>(&self) -> Scalar
    where
        Self: Sized,
    {
        U::from_canonical(self.canonical())
    }

    fn of<U: UnitOf<Self>>(value: Scalar) -> Self
    where
        Self: Sized,
    {
        Self::from_canonical(U::to_canonical(value))
    }
}
pub trait UnitOf<M: Measure> {
    fn from_canonical(canonical: Scalar) -> Scalar;
    fn to_canonical(converted: Scalar) -> Scalar;
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

macro_rules! measure_conversions {
    {} => {};
    {$self:ty,} => {};
    ($self:ty, Self * $rhs:ty => $output:ident in $output_unit:ty, $($rest:tt)*) => {
        impl core::ops::Mul<$rhs> for $self {
            type Output = $output;
            fn mul(self, rhs: $rhs) -> Self::Output {
                use $crate::private::MeasureInternal;
                $output::of::<$output_unit>(self.canonical() * rhs.canonical())
            }
        }

        $crate::measure_conversions!($self, $($rest)*);
    };
    ($self:ty, Self / $rhs:ty => $output:ident in $output_unit:ty, $($rest:tt)*) => {
        impl core::ops::Div<$rhs> for $self {
            type Output = $output;
            fn div(self, rhs: $rhs) -> Self::Output {
                use $crate::private::MeasureInternal;
                $output::of::<$output_unit>(self.canonical() / rhs.canonical())
            }
        }

        $crate::measure_conversions!($self, $($rest)*);
    };
}
pub(crate) use measure_conversions;

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
        $vis struct $name($crate::Scalar);

        impl $crate::private::Sealed for $name {}
        impl $crate::private::MeasureInternal for $name {
            fn canonical(&self) -> $crate::Scalar {
                self.0
            }
            fn from_canonical(value: $crate::Scalar) -> Self {
                Self(value)
            }
        }
        impl $crate::Measure for $name {}

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({} {})", stringify!($name), self.0, stringify!($canonical_unit))
            }
        }

        impl core::ops::Mul<$crate::Scalar> for $name {
            type Output = $name;
            fn mul(self, rhs: $crate::Scalar) -> $name {
                $name(self.0 * rhs)
            }
        }
        impl core::ops::MulAssign<$crate::Scalar> for $name {
            fn mul_assign(&mut self, rhs: $crate::Scalar) {
                self.0 *= rhs;
            }
        }
        impl core::ops::Mul<$name> for $crate::Scalar {
            type Output = $name;
            fn mul(self, rhs: $name) -> $name {
                $name(self * rhs.0)
            }
        }
        impl core::ops::Div<$crate::Scalar> for $name {
            type Output = $name;
            fn div(self, rhs: $crate::Scalar) -> $name {
                $name(self.0 / rhs)
            }
        }
        impl core::ops::DivAssign<$crate::Scalar> for $name {
            fn div_assign(&mut self, rhs: $crate::Scalar) {
                self.0 /= rhs;
            }
        }
        impl core::ops::Div<$name> for $crate::Scalar {
            type Output = $name;
            fn div(self, rhs: $name) -> $name {
                $name(self / rhs.0)
            }
        }

        impl core::ops::Add<$name> for $name {
            type Output = $name;
            fn add(self, rhs: $name) -> $name {
                $name(self.0 + rhs.0)
            }
        }
        impl core::ops::AddAssign<$name> for $name {
            fn add_assign(&mut self, rhs: $name) {
                self.0 += rhs.0;
            }
        }
        impl core::ops::Sub<$name> for $name {
            type Output = $name;
            fn sub(self, rhs: $name) -> $name {
                $name(self.0 - rhs.0)
            }
        }
        impl core::ops::SubAssign<$name> for $name {
            fn sub_assign(&mut self, rhs: $name) {
                self.0 -= rhs.0;
            }
        }

        $(
            $(#[$unit_meta])*
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
            $vis struct $unit;

            impl core::ops::Mul<$crate::Scalar> for $unit {
                type Output = $name;
                fn mul(self, rhs: $crate::Scalar) -> $name {
                    $name::of::<$unit>(rhs)
                }
            }
            impl core::ops::Mul<$unit> for $crate::Scalar {
                type Output = $name;
                fn mul(self, _rhs: $unit) -> $name {
                    $name::of::<$unit>(self)
                }
            }

            impl $unit {
                pub fn of(value: $crate::Scalar) -> $name {
                    $name::of::<Self>(value)
                }
            }

            $(
                impl $crate::UnitOf<$name> for $unit {
                    // $conversion = ratio of $unit to canonical unit
                    fn from_canonical(canonical: $crate::Scalar) -> $crate::Scalar {
                        canonical * $rhsper
                    }
                    fn to_canonical(converted: $crate::Scalar) -> $crate::Scalar {
                        converted / $rhsper
                    }
                }
            )?
            $(
                impl $crate::UnitOf<$name> for $unit {
                    // $conversion = ratio of $unit to canonical unit
                    fn from_canonical(canonical: $crate::Scalar) -> $crate::Scalar {
                        canonical / $lhsper
                    }
                    fn to_canonical(converted: $crate::Scalar) -> $crate::Scalar {
                        converted * $lhsper
                    }
                }
            )?
        )*

        $(
            impl $crate::UnitOf<$name> for $crate::Si {
                fn from_canonical(canonical: $crate::Scalar) -> $crate::Scalar {
                    $si_unit::from_canonical(canonical)
                }
                fn to_canonical(converted: $crate::Scalar) -> $crate::Scalar {
                    $si_unit::to_canonical(converted)
                }
            }
        )?

        $(
            $crate::measure_conversions!($name, $($converts)*);
        )?
    };
}
pub(crate) use measure;

macro_rules! extension_trait {
    (
        $(
            $func_name:ident => $measure:ident in $unit:ident
        ),*
    ) => {
        pub trait ScalarExt {
            $(
                fn $func_name(self) -> $measure;
            )*
        }
        impl ScalarExt for Scalar {
            $(
                fn $func_name(self) -> $measure {
                    $unit::of(self)
                }
            )*
        }
    };
}

extension_trait!(
    // Distance
    millimeters => Distance in Millimeters,
    centimeters => Distance in Centimeters,
    meters => Distance in Meters,
    kilometers => Distance in Kilometers,
    inches => Distance in Inches,
    feet => Distance in Feet,

    // Time
    seconds => Time in Seconds
);
