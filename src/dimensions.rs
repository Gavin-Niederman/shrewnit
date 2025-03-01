//! Default dimension implementations
//! 
//! Currently implemented:
//! 
//! - [`Length`]
//! - [`Area`]
//! - [`Volume`]
//! - [`Time`]
//! - [`LinearVelocity`]
//! - [`LinearAcceleration`]
//! - [`Angle`]
//! - [`AngularVelocity`]
//! - [`Force`]
//! - [`Mass`]
//! - [`Energy`]

use crate::dimension;

dimension!(
    /// Represents a distance.
    ///
    /// Canonically represented in meters.
    pub Length {
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
        Inches: 39.370079 per canonical,
        /// One foot.
        Feet: 3.2808399 per canonical,
        Yards: 1.0936133 per canonical,
        Miles: per 1609.344 canonical,
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

        SquareMillimeters: 1_000_000.0 per canonical,
        SquareCentimeters: 10_000.0 per canonical,
        SquareMeters: 1.0 per canonical,
        SquareKilometers: per 1_000_000.0 canonical,

        SquareInches: 1550.0031 per canonical,
        SquareFeet: 10.76391 per canonical,
        SquareYards: 1.19599 per canonical,
        Acres: per 4046.8564 canonical,
    } where {
        Self / Length => Length in Meters,
        Self * Length => Volume in CubicMeters,
    }
);

dimension!( 
    pub Volume {
        canonical: CubicMeters,

        Milliliters: 1_000_000.0 per canonical,
        Liters: 1000.0 per canonical,

        CubicMillimeters: 1_000_000_000.0 per canonical,
        CubicCentimeters: 1_000_000.0 per canonical,
        CubicMeters: 1.0 per canonical,
        CubicKilometers: per 1_000_000_000.0 canonical,

        CubicInches: 61023.744 per canonical,
        CubicFeet: 35.314667 per canonical,
        CubicYards: 1.3079506 per canonical,

        FluidOunces: 33814.023 per canonical,
        Pints: 2113.3764 per canonical,
        Quarts: 1056.6882 per canonical,
        Gallons: 264.17205 per canonical,

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
        Days: per 86_400.0 canonical,
        /// 604800 seconds.
        Weeks: per 604_800.0 canonical,
        /// 31536000 seconds.
        Years: per 31_556_926.0 canonical,
    } where {
        Self * LinearVelocity => Length in Meters,
        Self * LinearAcceleration => LinearVelocity in MetersPerSecond,
    }
);

dimension!(
    pub LinearVelocity {
        canonical: MetersPerSecond,

        MetersPerSecond: 1.0 per canonical,
        KilometersPerSecond: per 1000.0 canonical,
        KilometersPerHour: 3.6 per canonical,
        FeetPerSecond: 3.2808399 per canonical,
        MilesPerHour: 2.2369363 per canonical,
    } where {
        Self * Time => Length in Meters,
        Self / Time => LinearAcceleration in MetersPerSecondSquared,
    }
);
dimension! {
    pub LinearAcceleration {
        canonical: MetersPerSecondSquared,

        MetersPerSecondSquared: 1.0 per canonical,
        FeetPerSecondSquared: 3.2808399 per canonical,
    } where {
        Self * Time => LinearVelocity in MetersPerSecond,
        Self * Mass => Force in Newtons,
    }
}

dimension!(
    pub Angle {
        canonical: Radians,

        Radians: 1.0 per canonical,
        Rotations: per 6.2831853 canonical,
        Degrees: 57.29578 per canonical,
    } where {
        Self / Time => AngularVelocity in RadiansPerSecond,
    }
);
dimension!(
    pub AngularVelocity {
        canonical: RadiansPerSecond,

        RadiansPerSecond: 1.0 per canonical,
        RotationsPerSecond: per 6.2831853 canonical,
        RotationsPerMinute: 9.5492966 per canonical,
        DegreesPerSecond: 57.29578 per canonical,
    } where {
        Self * Time => Angle in Radians,
    }
);

dimension!(
    /// Represents mass.
    /// 
    /// Canonically represented in kilograms.
    pub Mass {
        canonical: Kilograms,

        Kilograms: 1.0 per canonical,
        Grams: 1_000.0 per canonical,
        Milligrams: 1_000_000.0 per canonical,
        Micrograms: 1_000_000_000.0 per canonical,
        
        Pounds: 2.2046226 per canonical,
        /// Represents the ounces unit of mass.
        Ounces: 35.273962 per canonical,
        Stones: per 6.3502932 canonical,

        /// Represents the tonne unit. Defined as one megagram.
        MetricTons: per 1000.0 canonical,
        /// Represents the American (short) ton. Defined as 2000 pounds.
        ShortTons: per 907.18474 canonical,
        /// Represents the British (long) ton. Defined as 2240 pounds.
        LongTons: per 1016.0469 canonical,
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

        Newtons: 1.0 per canonical,
        PoundsForce: 4.4482216 per canonical,
    } where {
        Self * Length => Energy in Joules,
    }
);

dimension!(
    /// Represents energy.
    /// 
    /// Canonically represented in joules.
    pub Energy {
        canonical: Joules,

        Joules: 1.0 per canonical,
        Calories: per 4.184 canonical,
        Kilocalories: per 4184.0 canonical,
    } where {
        Self / Length => Force in Newtons,
    }
);
