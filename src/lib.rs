//! Tiny **stable Rust** unit library built with macros.

#![no_std]

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

        measure_conversions!($self, $($rest)*);
    };
    ($self:ty, Self / $rhs:ty => $output:ident in $output_unit:ty, $($rest:tt)*) => {
        impl core::ops::Div<$rhs> for $self {
            type Output = $output;
            fn div(self, rhs: $rhs) -> Self::Output {
                use $crate::private::MeasureInternal;
                $output::of::<$output_unit>(self.canonical() / rhs.canonical())
            }
        }

        measure_conversions!($self, $($rest)*);
    };
}

macro_rules! measure {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            canonical: $canonical_unit:ident,

            $(
                $(#[$unit_meta:meta])*
                $unit:ident: $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?,
            )*
        } $(where {
            $($converts:tt)*
        })?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name(Scalar);

        impl $crate::private::Sealed for $name {}
        impl $crate::private::MeasureInternal for $name {
            fn canonical(&self) -> Scalar {
                self.0
            }
            fn from_canonical(value: Scalar) -> Self {
                Self(value)
            }
        }
        impl Measure for $name {}

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({} {})", stringify!($name), self.0, stringify!($canonical_unit))
            }
        }

        impl core::ops::Mul<Scalar> for $name {
            type Output = $name;
            fn mul(self, rhs: Scalar) -> $name {
                $name(self.0 * rhs)
            }
        }
        impl core::ops::MulAssign<Scalar> for $name {
            fn mul_assign(&mut self, rhs: Scalar) {
                self.0 *= rhs;
            }
        }
        impl core::ops::Mul<$name> for Scalar {
            type Output = $name;
            fn mul(self, rhs: $name) -> $name {
                $name(self * rhs.0)
            }
        }
        impl core::ops::Div<Scalar> for $name {
            type Output = $name;
            fn div(self, rhs: Scalar) -> $name {
                $name(self.0 / rhs)
            }
        }
        impl core::ops::DivAssign<Scalar> for $name {
            fn div_assign(&mut self, rhs: Scalar) {
                self.0 /= rhs;
            }
        }
        impl core::ops::Div<$name> for Scalar {
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

            impl core::ops::Mul<Scalar> for $unit {
                type Output = $name;
                fn mul(self, rhs: Scalar) -> $name {
                    $name::of::<$unit>(rhs)
                }
            }
            impl core::ops::Mul<$unit> for Scalar {
                type Output = $name;
                fn mul(self, _rhs: $unit) -> $name {
                    $name::of::<$unit>(self)
                }
            }

            impl $unit {
                pub fn of(value: Scalar) -> $name {
                    $name::of::<Self>(value)
                }
            }

            $(
                impl UnitOf<$name> for $unit {
                    // $conversion = ratio of $unit to canonical unit
                    fn from_canonical(canonical: Scalar) -> Scalar {
                        canonical * $rhsper
                    }
                    fn to_canonical(converted: Scalar) -> Scalar {
                        converted / $rhsper
                    }
                }
            )?
            $(
                impl UnitOf<$name> for $unit {
                    // $conversion = ratio of $unit to canonical unit
                    fn from_canonical(canonical: Scalar) -> Scalar {
                        canonical / $lhsper
                    }
                    fn to_canonical(converted: Scalar) -> Scalar {
                        converted * $lhsper
                    }
                }
            )?
        )*

        $(
            measure_conversions!($name, $($converts)*);
        )?
    };
}

measure!(
    /// Represents a distance.
    ///
    /// Canonically represented in meters.
    pub Distance {
        canonical: Meters,

        /// One millimeter.
        Millimeters: 1000.0 per canonical,
        /// One hundredth of a meter.
        Centimeters: 100.0 per canonical,
        /// One meter. The basic SI unit of distance.
        Meters: 1.0 per canonical,
        /// 1000 meters.
        Kilometers: per 1000.0 canonical,

        /// One inch.
        Inches: 39.3701 per canonical,
        /// One foot.
        Feet: 3.28084 per canonical,
    } where {
        Self / Time => LinearVelocity in MetersPerSecond,
    }
);

measure!(
    /// Represents a length of time.
    ///
    /// Canonically represented in seconds.
    pub Time {
        canonical: Seconds,

        /// One millionth of a second.
        Microseconds: 1_000_000.0 per canonical,
        /// One thousandth of a second.
        Milliseconds: 1000.0 per canonical,

        /// One second. The basic unit of time.
        Seconds: 1.0 per canonical,
        /// 60 seconds.
        Minutes: per 60.0 canonical,
        /// 3600 seconds.
        Hours: per 3600.0 canonical,

        /// 86400 seconds.
        Days: per 86400.0 canonical,
        /// 604800 seconds.
        Weeks: per 604800.0 canonical,
        /// 31536000 seconds.
        Years: per 31536000.0 canonical,
    } where {
        Self * LinearVelocity => Distance in Meters,
    }
);

measure!(
    pub LinearVelocity {
        canonical: MetersPerSecond,

        MetersPerSecond: 1.0 per canonical,
        KilometersPerHour: per 3.6 canonical,
        FeetPerSecond: 3.28084 per canonical,
        MilesPerHour: 2.23693629 per canonical,
    } where {
        Self * Time => Distance in Meters,
        Self / Time => LinearAcceleration in MetersPerSecondSquared,
    }
);
measure! {
    pub LinearAcceleration {
        canonical: MetersPerSecondSquared,

        MetersPerSecondSquared: 1.0 per canonical,
        FeetPerSecondSquared: 3.28084 per canonical,
    }
}