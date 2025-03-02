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
//! - [`Force`]
//! - [`Mass`]
//! - [`Torque`]
//! - [`Energy`]
//! 
//! If you need to define custom dimensions, you can use the [`dimension!`] macro.

use crate::dimension;

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
        Inches: 39.370079 per canonical,
        /// Represents the foot unit of length.
        Feet: 3.2808399 per canonical,
        /// Represents the yard unit of length.
        Yards: 1.0936133 per canonical,
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
        SquareInches: 1550.0031 per canonical,
        /// Represents the square foot unit of area.
        SquareFeet: 10.76391 per canonical,
        /// Represents the square yard unit of area.
        SquareYards: 1.19599 per canonical,
        /// Represents the acre unit of area.
        Acres: per 4046.8564 canonical,
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
        CubicInches: 61023.744 per canonical,
        /// Represents the cubic foot unit of volume.
        CubicFeet: 35.314667 per canonical,
        /// Represents the cubic yard unit of volume.
        CubicYards: 1.3079506 per canonical,

        /// Represents the fluid ounce unit of volume.
        FluidOunces: 33814.023 per canonical,
        /// Represents the pint unit of volume.
        Pints: 2113.3764 per canonical,
        /// Represents the quart unit of volume.
        Quarts: 1056.6882 per canonical,
        /// Represents the gallon unit of volume.
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
        FeetPerSecond: 3.2808399 per canonical,
        /// Represents the mile per hour unit of linear velocity.
        MilesPerHour: 2.2369363 per canonical,
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
        FeetPerSecondSquared: 3.2808399 per canonical,
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
        Rotations: per 6.2831853 canonical,
        /// Represents the degree unit of angle.
        Degrees: 57.29578 per canonical,
        /// Represents the gradian unit of angle.
        Gradians: 63.661977 per canonical,
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
        RotationsPerSecond: per 6.2831853 canonical,
        /// Represents the degree per second unit of angular velocity.
        RotationsPerMinute: 9.5492966 per canonical,
        /// Represents the degree per second unit of angular velocity.
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

        /// Represents the microgram unit of mass.
        Micrograms: 1_000_000_000.0 per canonical,
        /// Represents the milligram unit of mass.
        Milligrams: 1_000_000.0 per canonical,
        /// Represents the gram unit of mass.
        Grams: 1_000.0 per canonical,
        /// Represents the kilogram unit of mass.
        Kilograms: 1.0 per canonical,
        
        /// Represents the ton unit of mass.
        Pounds: 2.2046226 per canonical,
        /// Represents the ounces unit of mass.
        Ounces: 35.273962 per canonical,
        /// Represents the stone unit of mass.
        Stones: per 6.3502932 canonical,

        /// Represents the tonne unit of mass. Defined as one megagram.
        MetricTons: per 1000.0 canonical,
        /// Represents the American (short) ton unit of mass. Defined as 2000 pounds.
        ShortTons: per 907.18474 canonical,
        /// Represents the British (long) ton unit of mass. Defined as 2240 pounds.
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

        /// Represents the newton unit of force.
        /// This is the standard SI unit of force.
        Newtons: 1.0 per canonical,
        /// Represents the pound-force unit of force.
        PoundsForce: 4.4482216 per canonical,
    } where {
        Self * Length => Energy in Joules,
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
        canonical: NewtonMeterPerRadians,

        /// Represents the newton meter per radian unit of torque.
        NewtonMeterPerRadians: 1.0 per canonical,
        /// Represents the newton meter per degree unit of torque.
        NewtonMeterPerDegrees: per 57.29578 canonical,

        /// Represents the pound-foot per radian unit of torque.
        PoundFeetPerRadians: per 1.3558179 canonical,
        /// Represents the pound-foot per degree unit of torque.
        PoundFeetPerDegrees: per 77.682646 canonical,

        /// Represents the dyne centimeter per radian unit of torque.
        DyneCentimeterPerRadians: 10_000_000.0 per canonical,
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
    } where {
        Self / Length => Force in Newtons,
        Self / Angle => Torque in NewtonMeterPerRadians,
    }
);
