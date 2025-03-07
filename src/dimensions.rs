//! Default dimension implementations
//!
//! Units currently implemented by Shrewnit:
//!
//! - [`Length`]
//! - [`Area`]
//! - [`Volume`]
//! - [`Time`]
//! - [`LinearVelocity`]
//! - [`LinearAcceleration`]
//! - [`Angle`]
//! - [`AngularVelocity`]
//! - [`AngularAcceleration`]
//! - [`Force`]
//! - [`Pressure`]
//! - [`Mass`]
//! - [`Torque`]
//! - [`Energy`]
//! - [`Power`]
//! - [`Voltage`]
//! - [`Current`]
//! - [`Temperature`]
//!
//! If you need to define custom dimensions, you can use the [`dimension!`] macro.

use crate::{dimension, unit_type, Scalar, UnitOf};

dimension!(
    /// Represents a distance.
    ///
    /// Canonically represented in meters.
    pub Length {
        canonical: Meters,

        /// Represents the millimeter unit of length.
        Millimeters: 1000.0 per canonical,
        /// Represents the centimeter unit of length.
        Centimeters: 100.0 per canonical,
        /// Represents the meter unit of length.
        /// This is the standard SI unit of length.
        Meters: 1.0 per canonical,
        /// Represents the kilometer unit of length.
        Kilometers: per 1000.0 canonical,

        /// Represents the inch unit of length.
        Inches: 39.3700787401575 per canonical,
        /// Represents the foot unit of length.
        Feet: per 0.3048 canonical,
        /// Represents the yard unit of length.
        Yards: per 0.9144 canonical,
        /// Represents the mile unit of length.
        Miles: per 1609.344 canonical,
        /// Represents the nautical mile unit of length.
        NauticalMiles: per 1852.0 canonical,
    } where {
        Self / Time => LinearVelocity in MetersPerSecond,
        Self * Force => Energy in Joules,
        Self * Length => Area in SquareMeters,
        Self * Area => Volume in CubicMeters,
    }
);

dimension!(
    pub Area {
        canonical: SquareMeters,

        /// Represents the square millimeter unit of area.
        SquareMillimeters: 1_000_000.0 per canonical,
        /// Represents the square centimeter unit of area.
        SquareCentimeters: 10_000.0 per canonical,
        /// Represents the square meter unit of area.
        /// This is the standard SI unit of area.
        SquareMeters: 1.0 per canonical,
        /// Represents the square kilometer unit of area.
        SquareKilometers: per 1_000_000.0 canonical,

        /// Represents the square inch unit of area.
        SquareInches: 1550.0031000062 per canonical,
        /// Represents the square foot unit of area.
        SquareFeet: 10.7639104167097 per canonical,
        /// Represents the square yard unit of area.
        SquareYards: per 0.83612736 canonical,
        /// Represents the acre unit of area.
        Acres: per 4046.8564224 canonical,
    } where {
        Self / Length => Length in Meters,
        Self * Length => Volume in CubicMeters,
    }
);

dimension!(
    pub Volume {
        canonical: CubicMeters,

        /// Represents the cubic millimeter unit of volume.
        Milliliters: 1_000_000.0 per canonical,
        /// Represents the cubic centimeter unit of volume.
        Liters: 1000.0 per canonical,

        /// Represents the cubic meter unit of volume.
        CubicMillimeters: 1_000_000_000.0 per canonical,
        /// Represents the cubic meter unit of volume.
        CubicCentimeters: 1_000_000.0 per canonical,
        /// Represents the cubic meter unit of volume.
        CubicMeters: 1.0 per canonical,
        /// Represents the cubic kilometer unit of volume.
        CubicKilometers: per 1_000_000_000.0 canonical,

        /// Represents the cubic inch unit of volume.
        CubicInches: 61023.7440947323 per canonical,
        /// Represents the cubic foot unit of volume.
        CubicFeet: 35.3146667214886 per canonical,
        /// Represents the cubic yard unit of volume.
        CubicYards: 1.30795061931439 per canonical,

        /// Represents the fluid ounce unit of volume.
        FluidOunces: 33814.022701843 per canonical,
        /// Represents the pint unit of volume.
        Pints: 2113.37641886519 per canonical,
        /// Represents the quart unit of volume.
        Quarts: 1056.68820943259 per canonical,
        /// Represents the gallon unit of volume.
        Gallons: 264.172052358148 per canonical,

    } where {
        Self / Length => Area in SquareMeters,
        Self / Area => Length in Meters,
    }
);

dimension!(
    /// Represents a length of time.
    ///
    /// Canonically represented in seconds.
    pub Time {
        canonical: Seconds,

        /// Represents the millisecond unit of time.
        Microseconds: 1_000_000.0 per canonical,
        /// Represents the millisecond unit of time.
        Milliseconds: 1000.0 per canonical,

        /// Represents the second unit of time.
        /// This is the standard SI unit of time.
        Seconds: 1.0 per canonical,
        /// Represents the minute unit of time.
        Minutes: per 60.0 canonical,
        /// Represents the hour unit of time.
        Hours: per 3600.0 canonical,

        /// Represents the day unit of time.
        /// 86400 seconds.
        Days: per 86_400.0 canonical,
        /// Represents the week unit of time.
        /// 604800 seconds.
        Weeks: per 604_800.0 canonical,
        /// Represents the year unit of time.
        /// 31536000 seconds.
        Years: per 31_556_926.0 canonical,
    } where {
        Self * LinearVelocity => Length in Meters,
        Self * LinearAcceleration => LinearVelocity in MetersPerSecond,
        Self * AngularVelocity => Angle in Radians,
        Self * AngularAcceleration => AngularVelocity in RadiansPerSecond,
    }
);

dimension!(
    pub LinearVelocity {
        canonical: MetersPerSecond,

        /// Represents the meter per second unit of linear velocity.
        MetersPerSecond: 1.0 per canonical,
        /// Represents the kilometer per second unit of linear velocity.
        KilometersPerSecond: per 1000.0 canonical,
        /// Represents the kilometer per hour unit of linear velocity.
        KilometersPerHour: 3.6 per canonical,
        /// Represents the foot per second unit of linear velocity.
        FeetPerSecond: per 0.3048 canonical,
        /// Represents the mile per hour unit of linear velocity.
        MilesPerHour: per 0.44704 canonical,
    } where {
        Self * Time => Length in Meters,
        Self / Time => LinearAcceleration in MetersPerSecondSquared,
    }
);
dimension! {
    pub LinearAcceleration {
        canonical: MetersPerSecondSquared,

        /// Represents the meter per second squared unit of linear acceleration.
        MetersPerSecondSquared: 1.0 per canonical,
        /// Represents the foot per second squared unit of linear acceleration.
        FeetPerSecondSquared: per 0.3048 canonical,
    } where {
        Self * Time => LinearVelocity in MetersPerSecond,
        Self * Mass => Force in Newtons,
    }
}

dimension!(
    pub Angle {
        canonical: Radians,

        /// Represents the radian unit of angle.
        Radians: 1.0 per canonical,
        /// Represents the degree unit of angle.
        Rotations: per 6.28318530717959 canonical,
        /// Represents the degree unit of angle.
        Degrees: 57.2957795130823 per canonical,
        /// Represents the gradian unit of angle.
        Gradians: 63.6619772367581 per canonical,
    } where {
        Self / Time => AngularVelocity in RadiansPerSecond,
    }
);
dimension!(
    pub AngularVelocity {
        canonical: RadiansPerSecond,

        /// Represents the radian per second unit of angular velocity.
        RadiansPerSecond: 1.0 per canonical,
        /// Represents the rotation per second unit of angular velocity.
        RotationsPerSecond: per 6.28318530717959 canonical,
        /// Represents the degree per second unit of angular velocity.
        RotationsPerMinute: 9.54929658551372 per canonical,
        /// Represents the degree per second unit of angular velocity.
        DegreesPerSecond: 57.2957795130823 per canonical,
    } where {
        Self * Time => Angle in Radians,
    }
);
dimension!(
    pub AngularAcceleration {
        canonical: RadiansPerSecondSquared,

        /// Represents the radian per second squared unit of angular acceleration.
        RadiansPerSecondSquared: 1.0 per canonical,
        /// Represents the rotation per second squared unit of angular acceleration.
        RotationsPerSecondSquared: per 6.28318530717959 canonical,
        /// Represents the rotations per minute squared unit of angular acceleration.
        RotationsPerMinuteSquared: 572.957795130823 per canonical,
        /// Represents the degree per second squared unit of angular acceleration.
        DegreesPerSecondSquared: 57.2957795130823 per canonical,
    } where {
        Self * Time => AngularVelocity in RadiansPerSecond,
    }
);

dimension!(
    /// Represents mass.
    ///
    /// Canonically represented in kilograms.
    pub Mass {
        canonical: Kilograms,

        /// Represents the microgram unit of mass.
        Micrograms: 1_000_000_000.0 per canonical,
        /// Represents the milligram unit of mass.
        Milligrams: 1_000_000.0 per canonical,
        /// Represents the gram unit of mass.
        Grams: 1_000.0 per canonical,
        /// Represents the kilogram unit of mass.
        Kilograms: 1.0 per canonical,

        /// Represents the ton unit of mass.
        Pounds: per 0.45359237 canonical,
        /// Represents the ounces unit of mass.
        Ounces: 35.2739619495804 per canonical,
        /// Represents the stone unit of mass.
        Stones: per 6.35029318 canonical,

        /// Represents the tonne unit of mass. Defined as one megagram.
        MetricTons: per 1000.0 canonical,
        /// Represents the American (short) ton unit of mass. Defined as 2000 pounds.
        ShortTons: per 907.18474 canonical,
        /// Represents the British (long) ton unit of mass. Defined as 2240 pounds.
        LongTons: per 1016.0469088 canonical,
    } where {
        Self * LinearAcceleration => Force in Newtons,
    }
);

dimension!(
    /// Represents force.
    ///
    /// Canonically represented in newtons.
    pub Force {
        canonical: Newtons,

        /// Represents the newton unit of force.
        /// This is the standard SI unit of force.
        Newtons: 1.0 per canonical,
        /// Represents the pound-force unit of force.
        PoundsForce: 4.4482216 per canonical,
        /// Represents the dyne unit of force.
        Dynes: per 1e-05 canonical,
    } where {
        Self * Length => Energy in Joules,
        Self / LinearAcceleration => Mass in Kilograms,
        Self / Mass => LinearAcceleration in MetersPerSecondSquared,
        Self / Area => Pressure in Pascals,
    }
);

dimension!(
    /// Represents pressure.
    ///
    /// Canonically represented in pascals.
    pub Pressure {
        canonical: Pascals,

        /// Represents the pascal unit of pressure.
        /// This is the standard SI unit of pressure.
        Pascals: 1.0 per canonical,
        /// Represents the PSI (Pounds-force per Square Inch) unit of pressure.
        Psi: per 6894.75729316836 canonical,
        /// Represents the atmosphere unit of pressure.
        Atmospheres: per 101325.0 canonical,
        /// Represents the bar unit of pressure.
        Bars: per 100_000.0 canonical,
    } where {
        Self * Area => Force in Newtons,
    }
);

dimension!(
    /// Represents torque.
    ///
    /// # Note
    ///
    /// The units of this dimension are different from what you might expect.
    /// In the SI unit system, angle is not a base dimension, so torque is measured in N*m.
    /// However, Shrewnit makes angle a base dimension which means that torque is measured in N*m/rad.
    pub Torque {
        canonical: NewtonMetersPerRadian,

        /// Represents the newton meter per radian unit of torque.
        NewtonMetersPerRadian: 1.0 per canonical,
        /// Represents the newton meter per degree unit of torque.
        NewtonMetersPerDegree: per 57.2957795130823 canonical,

        /// Represents the pound-foot per radian unit of torque.
        PoundFeetPerRadian: per 1.3558179483314 canonical,
        /// Represents the pound-foot per degree unit of torque.
        PoundFeetPerDegree: per 77.6826462274756 canonical,

        /// Represents the dyne centimeter per radian unit of torque.
        DyneCentimetersPerRadians: 10_000_000.0 per canonical,
    } where {
        Self * Angle => Energy in Joules,
    }
);

dimension!(
    /// Represents energy.
    ///
    /// Canonically represented in joules.
    pub Energy {
        canonical: Joules,

        /// Represents the joule unit of energy.
        Joules: 1.0 per canonical,
        /// Represents the calorie unit of energy.
        Calories: per 4.184 canonical,
        /// Represents the kilocalorie unit of energy.
        Kilocalories: per 4184.0 canonical,
        /// Represents the erg unit of energy.
        Ergs: 10e-7 per canonical,
        /// Represents the watt-hour unit of energy.
        WattHours: per 3600.0 canonical,
    } where {
        Self / Length => Force in Newtons,
        Self / Angle => Torque in NewtonMetersPerRadian,
        Self / Time => Power in Watts,
    }
);

dimension!(
    /// Represents power.
    ///
    /// Canonically represented in watts.
    pub Power {
        canonical: Watts,

        /// Represents the watt unit of power.
        /// This is the standard SI unit of power.
        Watts: 1.0 per canonical,
        /// Represents the horsepower unit of power.
        Horsepower: per 745.69987158227 canonical,

        /// Represents the ergs per second unit of power.
        ErgsPerSecond: 10e-7 per canonical,

        /// Represents the foot-pounds per minute unit of power.
        FootPoundsPerMinute: 44.2537289566359 per canonical,
    } where {
        Self / Voltage => Current in Amperes,
        Self / Current => Voltage in Volts,
        Self * Time => Energy in Joules,
    }
);

dimension!(
    /// Represents voltage.
    ///
    /// Canonically represented in volts.
    pub Voltage {
        canonical: Volts,

        /// Represents the millivolt unit of voltage.
        Millivolts: 1000.0 per canonical,
        /// Represents the volt unit of voltage.
        /// This is the standard SI unit of voltage.
        Volts: 1.0 per canonical,
        /// Represents the kilovolt unit of voltage.
        Kilovolts: per 1000.0 canonical,
    } where {
        Self * Current => Power in Watts,
    }
);

dimension!(
    pub Current {
        canonical: Amperes,

        /// Represents the milliampere unit of current.
        Milliamperes: 1000.0 per canonical,
        /// Represents the ampere unit of current.
        /// This is the standard SI unit of current.
        Amperes: 1.0 per canonical,
        /// Represents the kiloampere unit of current.
        Kiloamperes: per 1000.0 canonical,
    } where {
        Self * Voltage => Power in Watts,
    }
);

dimension!(
    /// Represents temperature.
    pub Temperature {
        canonical: Kelvin,

        /// Represents the kelvin unit of temperature.
        Kelvin: 1.0 per canonical,
    }
);

unit_type!(
    /// Represents the celsius unit of temperature.
    pub Celsius of dimension Temperature
);
impl<S: Scalar> UnitOf<S, Temperature<S>> for Celsius {
    fn from_canonical(canonical: S) -> S {
        canonical + S::from_str("273.15").unwrap()
    }

    fn to_canonical(converted: S) -> S {
        converted - S::from_str("273.15").unwrap()
    }
}

unit_type!(
    /// Represents the celsius unit of temperature.
    pub Fahrenheit of dimension Temperature
);
impl<S: Scalar> UnitOf<S, Temperature<S>> for Fahrenheit {
    fn from_canonical(canonical: S) -> S {
        (canonical + S::from_str("459.67").unwrap()) * S::from_str("5.0").unwrap()
            / S::from_str("9.0").unwrap()
    }

    fn to_canonical(converted: S) -> S {
        (converted * S::from_str("9.0").unwrap() / S::from_str("5.0").unwrap())
            - S::from_str("273.15").unwrap()
    }
}
