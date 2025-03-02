use shrewnit::Dimension;

shrewnit::dimension!(
    Current {
        canonical: Amperes,

        Amperes: 1.0 per canonical,
    } where {
        Self * Voltage => Power in Watts,
    }
);

shrewnit::dimension!(
    Voltage {
        canonical: Volts,

        Volts: 1.0 per canonical,
    } where {
        Self * Current => Power in Watts,
    }
);

shrewnit::dimension!(
    Power {
        canonical: Watts,

        Watts: 1.0 per canonical,
    } where {
        Self / Voltage => Current in Amperes,
        Self / Current => Voltage in Volts,
    }
);


fn main() {
    let current = Amperes * 5.0;
    let voltage = Volts * 120.0;

    let power = current * voltage;

    println!("{}", power.to::<Watts>())
}
