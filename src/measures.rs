use crate::measure;

measure!(
    /// Represents a distance.
    ///
    /// Canonically represented in meters.
    pub Distance {
        si: Meters,
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
    } where {
        Self / Time => LinearVelocity in MetersPerSecond,
        Self * Force => Torque in NewtonMeters,
    }
);

measure!(
    /// Represents a length of time.
    ///
    /// Canonically represented in seconds.
    pub Time {
        si: Seconds,
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
        Self * LinearVelocity => Distance in Meters,
        Self * LinearAcceleration => LinearVelocity in MetersPerSecond,
    }
);

measure!(
    pub LinearVelocity {
        si: MetersPerSecond,
        canonical: MetersPerSecond,

        MetersPerSecond: 1.0 per canonical,
        KilometersPerHour: 3.6 per canonical,
        FeetPerSecond: 3.2808399 per canonical,
        MilesPerHour: 2.2369363 per canonical,
    } where {
        Self * Time => Distance in Meters,
        Self / Time => LinearAcceleration in MetersPerSecondSquared,
    }
);
measure! {
    pub LinearAcceleration {
        si: MetersPerSecondSquared,
        canonical: MetersPerSecondSquared,

        MetersPerSecondSquared: 1.0 per canonical,
        FeetPerSecondSquared: 3.2808399 per canonical,
    } where {
        Self * Time => LinearVelocity in MetersPerSecond,
    }
}

measure!(
    pub Angle {
        si: Radians,
        canonical: Radians,

        Radians: 1.0 per canonical,
        Rotations: per 6.2831853 canonical,
        Degrees: 57.29578 per canonical,
    } where {
        Self / Time => AngularVelocity in RadiansPerSecond,
    }
);
measure!(
    pub AngularVelocity {
        si: RadiansPerSecond,
        canonical: RadiansPerSecond,

        RadiansPerSecond: 1.0 per canonical,
        RotationsPerSecond: per 6.2831853 canonical,
        RotationsPerMinute: 9.5492966 per canonical,
        DegreesPerSecond: 57.29578 per canonical,
    } where {
        Self * Time => Angle in Radians,
    }
);

measure!(
    pub Force {
        si: Newtons,
        canonical: Newtons,

        Newtons: 1.0 per canonical,
        Pounds: 4.4482216 per canonical,
    } where {
        Self * Distance => Torque in NewtonMeters,
    }
);

measure!(
    pub Torque {
        si: NewtonMeters,
        canonical: NewtonMeters,

        NewtonMeters: 1.0 per canonical,
        FootPounds: per 1.3558179 canonical,
    } where {
        Self / Distance => Force in Newtons,
        Self / Force => Distance in Meters,
    }
);
