use shrewnit::{Distance, Feet, Inches, Measure, Scalar, Seconds, UnitOf};

struct HalfInches;
impl UnitOf<Distance> for HalfInches {
    fn to_canonical(converted: Scalar) -> Scalar {
        Inches::to_canonical(converted) * 2.0
    }
    fn from_canonical(canonical: Scalar) -> Scalar {
        Inches::from_canonical(canonical * 2.0)
    }
}

fn main() {
    let distance = 30.0 * Feet;
    let time = 3.5 * Seconds;

    let average_velocity = distance / time;
    let acceleration = average_velocity / time;

    println!("{:?} {:?}", average_velocity, acceleration);

    let half_inches = distance.to::<HalfInches>();
    println!("{:?}", half_inches);
}
